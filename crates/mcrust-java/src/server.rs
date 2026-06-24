use std::net::SocketAddr;
use std::sync::Arc;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use crate::protocol::{
    decode_handshake, decode_ping_payload, encode_pong, encode_status_response,
    is_supported_protocol, read_packets_from_buffer,     Handshake, StatusDescription, StatusPlayers,
    StatusResponse, StatusVersion, HANDSHAKE_ID, PING_ID, STATE_HANDSHAKE, STATE_LOGIN,
    STATE_STATUS, STATUS_REQUEST_ID,
};
use crate::JavaError;

#[derive(Clone)]
pub struct JavaStatusConfig {
    pub motd: String,
    pub max_players: u32,
    pub online_players: u32,
    /// Advertised protocol when client version unknown
    pub default_protocol: i32,
}

pub async fn run_java_listener(
    bind: SocketAddr,
    cfg: Arc<JavaStatusConfig>,
) -> Result<(), JavaError> {
    let listener = TcpListener::bind(bind).await?;
    tracing::info!(%bind, "java listener ready");
    loop {
        let (stream, peer) = listener.accept().await?;
        let cfg = cfg.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, peer, cfg).await {
                tracing::debug!(%peer, error = %e, "java connection closed");
            }
        });
    }
}

async fn handle_connection(
    mut stream: TcpStream,
    peer: SocketAddr,
    cfg: Arc<JavaStatusConfig>,
) -> Result<(), JavaError> {
    let mut buf = Vec::new();
    let mut handshake: Option<Handshake> = None;
    let mut state = STATE_HANDSHAKE;

    loop {
        let mut chunk = [0u8; 4096];
        let n = stream.read(&mut chunk).await?;
        if n == 0 {
            return Ok(());
        }
        buf.extend_from_slice(&chunk[..n]);

        let packets = read_packets_from_buffer(&mut buf)?;
        for (id, payload) in packets {
            match state {
                STATE_HANDSHAKE => {
                    if id != HANDSHAKE_ID {
                        return Err(JavaError::Protocol("expected handshake".into()));
                    }
                    let (hs, _) = decode_handshake(&payload)?;
                    if hs.next_state != STATE_STATUS && hs.next_state != STATE_LOGIN {
                        return Err(JavaError::Protocol("unsupported next state".into()));
                    }
                    if !is_supported_protocol(hs.protocol_version) {
                        tracing::info!(
                            %peer,
                            protocol = hs.protocol_version,
                            "unsupported java protocol (closing)"
                        );
                        return Ok(());
                    }
                    state = hs.next_state;
                    handshake = Some(hs);
                }
                STATE_STATUS => {
                    let hs = handshake.as_ref().expect("handshake");
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
                        _ => {
                            return Err(JavaError::Protocol(format!(
                                "unexpected status packet {id}"
                            )));
                        }
                    }
                }
                STATE_LOGIN => {
                    tracing::debug!(%peer, "login not implemented yet");
                    return Ok(());
                }
                _ => return Err(JavaError::Protocol("bad state".into())),
            }
        }
    }
}
