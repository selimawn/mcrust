mod config;

use std::env;
use std::net::{Ipv4Addr, SocketAddr};
use std::path::PathBuf;
use std::sync::Arc;

use mcrust_bedrock::BedrockPingConfig;
use mcrust_java::JavaStatusConfig;
use tracing::info;
use tracing_subscriber::EnvFilter;

use config::ServerConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env().add_directive("mcrust=info".parse().unwrap()),
        )
        .init();

    let conf_path = env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("conf.txt"));

    let cfg = match ServerConfig::load(&conf_path) {
        Ok(c) => c,
        Err(e) => {
            tracing::warn!(path = %conf_path.display(), error = %e, "using defaults (missing or invalid conf.txt)");
            ServerConfig::default()
        }
    };

    let java_bind = SocketAddr::from((Ipv4Addr::UNSPECIFIED, cfg.server_port));
    let java_cfg = Arc::new(JavaStatusConfig {
        motd: cfg.motd.clone(),
        max_players: cfg.max_players,
        online_players: 0,
        default_protocol: 767,
    });

    let java = tokio::spawn(async move {
        if let Err(e) = mcrust_java::run_java_listener(java_bind, java_cfg).await {
            tracing::error!(error = %e, "java listener exited");
        }
    });

    let bedrock = if cfg.enable_bedrock {
        let bedrock_bind = SocketAddr::from((Ipv4Addr::UNSPECIFIED, cfg.bedrock_port));
        let bedrock_cfg = Arc::new(BedrockPingConfig {
            motd_line: cfg.motd.clone(),
            server_guid: 0x2D05F9D2E7BC4A1Fu64,
            max_players: cfg.max_players,
            online_players: 0,
        });
        Some(tokio::spawn(async move {
            if let Err(e) = mcrust_bedrock::run_bedrock_ping(bedrock_bind, bedrock_cfg).await {
                tracing::error!(error = %e, "bedrock listener exited");
            }
        }))
    } else {
        None
    };

    info!(
        name = %cfg.server_name,
        java_port = cfg.server_port,
        bedrock = cfg.enable_bedrock,
        bedrock_port = cfg.bedrock_port,
        online_mode = cfg.online_mode,
        bedrock_online_mode = cfg.bedrock_online_mode,
        "mcrust running (status / ping only; login P5/P6 next)"
    );

    tokio::signal::ctrl_c().await?;
    info!("shutting down");
    java.abort();
    if let Some(b) = bedrock {
        b.abort();
    }
    Ok(())
}
