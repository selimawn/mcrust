use std::net::SocketAddr;
use std::sync::Arc;

use mcrust_bridge::BridgeRouter;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use crate::connection::handle_java_connection;
use crate::crypto::ServerKeys;
use crate::protocol::{
    decode_handshake, decode_ping_payload, encode_pong, encode_status_response,
    read_packets_from_buffer, Handshake, StatusDescription, StatusPlayers,
    StatusResponse, StatusVersion, HANDSHAKE_ID, PING_ID, STATE_STATUS, STATUS_REQUEST_ID,
};
use crate::JavaError;

#[derive(Clone)]
pub struct JavaStatusConfig {
    pub motd: String,
    pub max_players: u32,
    pub online_players: u32,
}

#[derive(Clone)]
pub struct JavaServerConfig {
    pub status: JavaStatusConfig,
    pub play: JavaPlayConfig,
}

#[derive(Clone)]
pub struct JavaPlayConfig {
    pub online_mode: bool,
    pub prevent_proxy_connections: bool,
}

pub async fn run_java_listener(
    bind: SocketAddr,
    cfg: Arc<JavaServerConfig>,
    router: BridgeRouter,
    keys: Arc<ServerKeys>,
    http: reqwest::Client,
) -> Result<(), JavaError> {
    let listener = TcpListener::bind(bind).await?;
    tracing::info!(%bind, "java listener ready");
    loop {
        let (stream, peer) = listener.accept().await?;
        let cfg = cfg.clone();
        let router = router.clone();
        let keys = keys.clone();
        let http = http.clone();
        tokio::spawn(async move {
            if let Err(e) =
                handle_java_connection(
                    stream,
                    peer,
                    Arc::new(cfg.play.clone()),
                    cfg.status.clone(),
                    router,
                    keys,
                    http,
                )
                    .await
            {
                tracing::debug!(%peer, error = %e, "java connection ended");
            }
        });
    }
}

pub async fn handle_status_on_stream(
    stream: &mut TcpStream,
    hs: &Handshake,
    cfg: &JavaStatusConfig,
) -> Result<(), JavaError> {
    let mut buf = Vec::new();
    loop {
        let mut chunk = [0u8; 4096];
        let n = stream.read(&mut chunk).await?;
        if n == 0 {
            return Ok(());
        }
        buf.extend_from_slice(&chunk[..n]);
        let packets = read_packets_from_buffer(&mut buf)?;
        for (id, payload) in packets {
            match id {
                STATUS_REQUEST_ID => {
                    let version_label = crate::protocol::version_name(hs.protocol_version)
                        .unwrap_or("mcrust");
                    let status = StatusResponse {
                        version: StatusVersion {
                            name: version_label,
                            protocol: hs.protocol_version,
                        },
                        players: StatusPlayers {
                            max: cfg.max_players,
                            online: cfg.online_players,
                        },
                        description: StatusDescription { text: &cfg.motd },
                    };
                    let json = serde_json::to_string(&status)?;
                    stream.write_all(&encode_status_response(&json)).await?;
                }
                PING_ID => {
                    let _ = decode_ping_payload(&payload)?;
                    stream.write_all(&encode_pong(&payload)).await?;
                    return Ok(());
                }
                _ => {}
            }
        }
    }
}