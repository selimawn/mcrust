use std::net::SocketAddr;
use std::sync::Arc;

use crossbeam_channel::Sender;
use mcrust_bridge::BridgeRouter;
use mcrust_protocol::{Gamemode, InboundEvent, JoinParams, OutboundCommand, Platform, PlayerId};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::auth::{format_uuid_with_dashes, has_joined, minecraft_server_hash, offline_uuid};
use crate::crypto::{random_bytes, AesStream, ServerKeys};
use crate::login::{
    decode_encryption_response, decode_login_start, encode_encryption_request, encode_login_success,
    ENCRYPTION_RESPONSE, LOGIN_START,
};
use crate::play;
use crate::protocol::{decode_handshake, read_packets_from_buffer, STATE_LOGIN, STATE_STATUS};
use crate::JavaError;

pub async fn handle_java_connection(
    mut stream: TcpStream,
    peer: SocketAddr,
    cfg: Arc<crate::server::JavaPlayConfig>,
    status: crate::server::JavaStatusConfig,
    router: BridgeRouter,
    keys: Arc<ServerKeys>,
    http: reqwest::Client,
) -> Result<(), JavaError> {
    let mut buf = Vec::new();
    let mut aes: Option<AesStream> = None;
    let mut player_id: Option<PlayerId> = None;
    let mut entity_id = (rand::random::<u32>() & 0x7fff_ffff) as i32;
    let mut username = String::new();
    let mut verify_token = Vec::new();
    let mut login_done = false;
    let mut out_rx: Option<crossbeam_channel::Receiver<OutboundCommand>> = None;

    loop {
        if let Some(rx) = out_rx.as_ref() {
            while let Ok(cmd) = rx.try_recv() {
                if let Some(mut data) = encode_command(&cmd) {
                    if let Some(a) = aes.as_mut() {
                        a.encrypt(&mut data);
                    }
                    stream.write_all(&data).await?;
                }
            }
        }

        let mut chunk = [0u8; 8192];
        let n = stream.read(&mut chunk).await?;
        if n == 0 {
            break;
        }
        let mut raw = chunk[..n].to_vec();
        if let Some(a) = aes.as_mut() {
            a.decrypt(&mut raw);
        }
        buf.extend_from_slice(&raw);

        let packets = read_packets_from_buffer(&mut buf)?;
        for (id, payload) in packets {
            if !login_done && player_id.is_none() && id == crate::protocol::HANDSHAKE_ID {
                let (hs, _) = decode_handshake(&payload)?;
                if hs.next_state == STATE_STATUS {
                    return crate::server::handle_status_on_stream(&mut stream, &hs, &status).await;
                }
                if hs.next_state != STATE_LOGIN {
                    return Ok(());
                }
                continue;
            }

            if !login_done {
                if id == LOGIN_START {
                    let (name, _) = decode_login_start(&payload)?;
                    username = name;
                    if cfg.online_mode {
                        verify_token = random_bytes(16);
                        let pkt = encode_encryption_request("", &keys.public_key_der, &verify_token, true);
                        stream.write_all(&pkt).await?;
                    } else {
                        let uuid = offline_uuid(&username);
                        let (tx, pid, eid) = complete_login(
                            &router,
                            &username,
                            uuid,
                            &mut stream,
                            &mut aes,
                        )
                        .await?;
                        out_rx = Some(tx);
                        player_id = Some(pid);
                        entity_id = eid;
                        login_done = true;
                    }
                } else if id == ENCRYPTION_RESPONSE && cfg.online_mode {
                    let (enc_secret, enc_token) = decode_encryption_response(&payload)?;
                    let secret = keys
                        .decrypt_pair(&enc_secret, &enc_token, &verify_token)
                        .map_err(JavaError::protocol)?;
                    let server_id = minecraft_server_hash("", &secret, &keys.public_key_der);
                    let ip = cfg
                        .prevent_proxy_connections
                        .then(|| peer.ip().to_string());
                    let profile = has_joined(
                        &http,
                        &username,
                        &server_id,
                        ip.as_deref(),
                    )
                    .await
                    .map_err(|e| JavaError::Protocol(e.to_string()))?;
                    let Some(profile) = profile else {
                        stream
                            .write_all(&play::disconnect_login("Failed to verify username"))
                            .await?;
                        return Ok(());
                    };
                    let uuid = format_uuid_with_dashes(&profile.id);
                    aes = Some(AesStream::from_secret(secret));
                    let (tx, pid, eid) = complete_login(
                        &router,
                        &profile.name,
                        uuid,
                        &mut stream,
                        &mut aes,
                    )
                    .await?;
                    out_rx = Some(tx);
                    player_id = Some(pid);
                    entity_id = eid;
                    login_done = true;
                }
                continue;
            }

            if let Some(pid) = player_id {
                match id {
                    0x1b if payload.len() >= 8 => {
                        let ka = i64::from_be_bytes(payload[0..8].try_into().unwrap());
                        router.forward_inbound(InboundEvent::KeepAliveAck {
                            player_id: pid,
                            payload: ka,
                        });
                    }
                    0x1a => {
                        if let Some((x, y, z, yaw, pitch, on_ground)) = decode_pos(&payload) {
                            router.forward_inbound(InboundEvent::PlayerInput {
                                player_id: pid,
                                x,
                                y,
                                z,
                                yaw,
                                pitch,
                                on_ground,
                            });
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    if let Some(pid) = player_id {
        router.unregister_session(pid);
    }
    Ok(())
}

async fn complete_login(
    router: &BridgeRouter,
    name: &str,
    uuid: uuid::Uuid,
    stream: &mut TcpStream,
    aes: &mut Option<AesStream>,
) -> Result<(crossbeam_channel::Receiver<OutboundCommand>, PlayerId, i32), JavaError> {
    let (session_tx, session_rx) = crossbeam_channel::unbounded();
    let join = JoinParams {
        name: name.to_string(),
        uuid,
        platform: Platform::Java,
        xuid: None,
        gamemode: Gamemode::Creative,
    };
    let pid = router.player_join(join, session_tx);
    let entity_id = (rand::random::<u32>() & 0x7fff_ffff) as i32;

    async fn write_pkt(
        stream: &mut TcpStream,
        aes: &mut Option<AesStream>,
        data: Vec<u8>,
    ) -> Result<(), JavaError> {
        let mut data = data;
        if let Some(a) = aes.as_mut() {
            a.encrypt(&mut data);
        }
        stream.write_all(&data).await?;
        Ok(())
    }

    write_pkt(stream, aes, encode_login_success(&uuid.to_string(), name)).await?;
    if aes.is_none() {
        write_pkt(stream, aes, play::set_compression(256)).await?;
    }
    write_pkt(stream, aes, play::finish_configuration()).await?;
    write_pkt(stream, aes, play::login_play_join_game(entity_id)).await?;
    write_pkt(
        stream,
        aes,
        play::synchronize_position(0.5, 64.0, 0.5, 0.0, 0.0),
    )
    .await?;

    Ok((session_rx, pid, entity_id))
}

fn decode_pos(payload: &[u8]) -> Option<(f64, f64, f64, f32, f32, bool)> {
    if payload.len() < 33 {
        return None;
    }
    Some((
        f64::from_be_bytes(payload[0..8].try_into().ok()?),
        f64::from_be_bytes(payload[8..16].try_into().ok()?),
        f64::from_be_bytes(payload[16..24].try_into().ok()?),
        f32::from_be_bytes(payload[24..28].try_into().ok()?),
        f32::from_be_bytes(payload[28..32].try_into().ok()?),
        payload[32] != 0,
    ))
}

fn encode_command(cmd: &OutboundCommand) -> Option<Vec<u8>> {
    match cmd {
        OutboundCommand::KeepAlive { payload, .. } => Some(play::keep_alive(*payload)),
        OutboundCommand::TeleportPlayer {
            position,
            yaw,
            pitch,
            ..
        } => Some(play::synchronize_position(
            position.x,
            position.y,
            position.z,
            *yaw,
            *pitch,
        )),
        OutboundCommand::BroadcastMovement {
            position,
            yaw,
            pitch,
            on_ground,
            ..
        } => Some(play::player_position_sync(
            position.x,
            position.y,
            position.z,
            *yaw,
            *pitch,
            *on_ground,
        )),
        OutboundCommand::Disconnect { reason, .. } => Some(play::disconnect_login(reason)),
        _ => None,
    }
}