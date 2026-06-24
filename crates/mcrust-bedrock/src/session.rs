use std::sync::Arc;

use crossbeam_channel::Receiver;
use mcrust_bridge::BridgeRouter;
use mcrust_protocol::{Gamemode, InboundEvent, JoinParams, OutboundCommand, Platform, PlayerId};
use rust_raknet::Reliability;
use tracing::{info, warn};

use crate::auth::{handshake_jwt_offline, parse_connection_request, verify_login_chain};
use crate::codec::{decode_batch_payload, encode_batch, parse_gamepacket};
use crate::config::BedrockPlayConfig;
use crate::packets::{
    self, move_player, play_status, resource_pack_stack, resource_packs_info,
    server_handshake, set_local_player_initialized,
    PACKET_REQUEST_NETWORK_SETTINGS,
};


pub const SUPPORTED_PROTOCOLS: &[i32] = &[685, 686, 687, 688];

pub struct BedrockSessionState {
    pub player_id: Option<PlayerId>,
    pub display_name: String,
    pub runtime_id: u64,
    pub unique_id: i64,
    pub out_rx: Option<Receiver<OutboundCommand>>,
    pub compress: bool,
    pub spawned: bool,
}

impl Default for BedrockSessionState {
    fn default() -> Self {
        Self {
            player_id: None,
            display_name: "Player".into(),
            runtime_id: (rand::random::<u32>() as u64) | 1,
            unique_id: rand::random::<i64>(),
            out_rx: None,
            compress: false,
            spawned: false,
        }
    }
}

pub async fn handle_raknet_session(
    mut socket: rust_raknet::RaknetSocket,
    router: BridgeRouter,
    cfg: Arc<BedrockPlayConfig>,
) {
    let mut state = BedrockSessionState::default();
    loop {
        if let Some(rx) = state.out_rx.as_ref() {
            while let Ok(cmd) = rx.try_recv() {
                if let Some(batch) = encode_outbound(&cmd, &state) {
                    let _ = socket
                        .send(&batch, Reliability::ReliableOrdered)
                        .await;
                }
            }
        }

        let data = match socket.recv().await {
            Ok(d) => d,
            Err(_) => break,
        };
        if let Err(e) = process_payload(&data, &mut socket, &router, &cfg, &mut state).await {
            warn!(error = %e, "bedrock session error");
            break;
        }
    }
    if let Some(pid) = state.player_id {
        router.unregister_session(pid);
    }
}

async fn process_payload(
    data: &[u8],
    socket: &mut rust_raknet::RaknetSocket,
    router: &BridgeRouter,
    cfg: &BedrockPlayConfig,
    state: &mut BedrockSessionState,
) -> Result<(), String> {
    let packets = decode_batch_payload(data, state.compress)?;
    for pkt in packets {
        let (id, payload) = parse_gamepacket(&pkt)?;
        match id {
            packets::PACKET_REQUEST_NETWORK_SETTINGS => {
                let proto = crate::codec::read_request_network_settings(&payload)?;
                if !SUPPORTED_PROTOCOLS.contains(&proto) {
                    return Err(format!("unsupported protocol {proto}"));
                }
                send_batch(socket, &[packets::network_settings(256, crate::codec::COMPRESSION_NONE)])
                    .await?;
            }
            packets::PACKET_LOGIN => {
                let conn = if payload.len() > 4 {
                    let proto = i32::from_be_bytes(payload[0..4].try_into().unwrap());
                    let _ = proto;
                    &payload[4..]
                } else {
                    &payload[..]
                };
                let (chain, client_jwt) = parse_connection_request(conn)?;
                let identity = verify_login_chain(&chain, cfg.online_mode)?;
                if let Some(ref key) = identity.identity_public_key {
                    let _ = crate::jwt_auth::verify_client_data_jwt(&client_jwt, key);
                }
                let (tx, rx) = crossbeam_channel::unbounded();
                let join = JoinParams {
                    name: identity.display_name.clone(),
                    uuid: identity.uuid,
                    platform: Platform::Bedrock,
                    xuid: identity.xuid.clone(),
                    gamemode: Gamemode::Creative,
                };
                let pid = router.player_join(join, tx);
                state.player_id = Some(pid);
                state.display_name = identity.display_name.clone();
                state.out_rx = Some(rx);
                info!(
                    name = %identity.display_name,
                    online = identity.online,
                    "bedrock login accepted"
                );
                let hs = server_handshake(&handshake_jwt_offline());
                send_batch(socket, &[hs]).await?;
            }
            0x04 => {
                send_batch(
                    socket,
                    &[
                        play_status(0),
                        resource_packs_info(),
                        resource_pack_stack(),
                    ],
                )
                .await?;
            }
            0x08 | 0x56 => {
                if state.spawned {
                    continue;
                }
                state.spawned = true;
                let sg = crate::start_game::encode_start_game(
                    state.unique_id,
                    state.runtime_id,
                    &state.display_name,
                    &cfg.motd,
                    0.5,
                    64.0,
                    0.5,
                    "1.21.50",
                );
                send_batch(
                    socket,
                    &[
                        play_status(3),
                        sg,
                        set_local_player_initialized(state.runtime_id),
                    ],
                )
                .await?;
            }
            0x94 => {
                if let Some(pid) = state.player_id {
                    if payload.len() >= 20 {
                        let x = f32::from_le_bytes(payload[4..8].try_into().unwrap());
                        let y = f32::from_le_bytes(payload[8..12].try_into().unwrap());
                        let z = f32::from_le_bytes(payload[12..16].try_into().unwrap());
                        router.forward_inbound(InboundEvent::PlayerInput {
                            player_id: pid,
                            x: x as f64,
                            y: y as f64,
                            z: z as f64,
                            yaw: 0.0,
                            pitch: 0.0,
                            on_ground: true,
                        });
                    }
                }
            }
            _ => {}
        }
    }
    Ok(())
}

async fn send_batch(socket: &mut rust_raknet::RaknetSocket, packets: &[Vec<u8>]) -> Result<(), String> {
    let batch = encode_batch(packets, false);
    socket
        .send(&batch, Reliability::ReliableOrdered)
        .await
        .map_err(|e| format!("{e:?}"))
}

fn encode_outbound(cmd: &OutboundCommand, state: &BedrockSessionState) -> Option<Vec<u8>> {
    match cmd {
        OutboundCommand::BroadcastMovement {
            position,
            yaw,
            pitch,
            on_ground,
            ..
        } => {
            let pkt = move_player(
                state.runtime_id,
                position.x as f32,
                position.y as f32,
                position.z as f32,
                *yaw,
                *pitch,
                *on_ground,
            );
            Some(encode_batch(&[pkt], false))
        }
        OutboundCommand::TeleportPlayer {
            position,
            yaw,
            pitch,
            ..
        } => {
            let pkt = move_player(
                state.runtime_id,
                position.x as f32,
                position.y as f32,
                position.z as f32,
                *yaw,
                *pitch,
                true,
            );
            Some(encode_batch(&[pkt], false))
        }
        _ => None,
    }
}