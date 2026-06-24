//! Bedrock play path (offline / LAN milestone).
//! Full online JWT + RakNet session: extend with `rust-raknet` connected flow.

use std::net::SocketAddr;
use std::sync::Arc;

use mcrust_bridge::BridgeRouter;
use mcrust_protocol::{Gamemode, JoinParams, OutboundCommand, Platform};
use tracing::info;
use uuid::Uuid;

#[derive(Clone)]
pub struct BedrockPlayConfig {
    pub online_mode: bool,
    pub motd: String,
}

pub fn offline_join_player(router: &BridgeRouter, name: &str) -> mcrust_protocol::PlayerId {
    let (tx, _rx) = crossbeam_channel::unbounded::<OutboundCommand>();
    let join = JoinParams {
        name: name.to_string(),
        uuid: Uuid::new_v4(),
        platform: Platform::Bedrock,
        xuid: None,
        gamemode: Gamemode::Creative,
    };
    router.player_join(join, tx)
}

pub async fn run_bedrock_hybrid(
    bind: SocketAddr,
    ping_cfg: Arc<crate::server::BedrockPingConfig>,
    play_cfg: Arc<BedrockPlayConfig>,
    router: BridgeRouter,
) -> std::io::Result<()> {
    use tokio::net::UdpSocket;
    let sock = UdpSocket::bind(bind).await?;
    info!(%bind, "bedrock udp (ping + offline play stub)");
    let mut buf = [0u8; 2048];
    loop {
        let (len, from) = sock.recv_from(&mut buf).await?;
        let data = &buf[..len];
        if crate::ping::is_unconnected_ping(data) {
            if let Some(time) = crate::ping::parse_ping_time(data) {
                let pong = crate::ping::build_unconnected_pong(
                    time,
                    ping_cfg.server_guid,
                    &ping_cfg.formatted_motd(),
                );
                let _ = sock.send_to(&pong, from).await;
            }
            continue;
        }
        if !play_cfg.online_mode && data.len() > 2 && data[0] == 0xfe && data[1] == 0x01 {
            let name = format!("Bedrock_{}", from.port());
            let pid = offline_join_player(&router, &name);
            info!(%from, id = pid.0, "bedrock offline stub join (experimental)");
        }
    }
}