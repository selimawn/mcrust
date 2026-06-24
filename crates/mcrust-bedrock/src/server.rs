use std::net::SocketAddr;
use std::sync::Arc;

use tokio::net::UdpSocket;

use crate::ping::{build_unconnected_pong, is_unconnected_ping, parse_ping_time};

#[derive(Clone)]
pub struct BedrockPingConfig {
    pub motd_line: String,
    pub server_guid: u64,
    pub max_players: u32,
    pub online_players: u32,
}

impl BedrockPingConfig {
    /// Classic MCPE server list string (simplified).
    pub fn formatted_motd(&self) -> String {
        format!(
            "MCPE;{};;1;{};{};0;0;mcrust;0;",
            self.motd_line.replace(';', ","),
            self.online_players,
            self.max_players
        )
    }
}

pub async fn run_bedrock_ping(
    bind: SocketAddr,
    cfg: Arc<BedrockPingConfig>,
) -> std::io::Result<()> {
    let sock = UdpSocket::bind(bind).await?;
    tracing::info!(%bind, "bedrock udp listener ready (unconnected ping/pong)");
    let mut buf = [0u8; 2048];
    loop {
        let (len, from) = sock.recv_from(&mut buf).await?;
        let data = &buf[..len];
        if is_unconnected_ping(data) {
            let Some(time) = parse_ping_time(data) else {
                continue;
            };
            let pong = build_unconnected_pong(time, cfg.server_guid, &cfg.formatted_motd());
            let _ = sock.send_to(&pong, from).await;
            tracing::trace!(%from, "unconnected pong");
        }
    }
}
