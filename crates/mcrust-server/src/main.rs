mod config;

use std::env;
use std::net::{Ipv4Addr, SocketAddr};
use std::path::PathBuf;
use std::sync::Arc;

use mcrust_bedrock::{BedrockPingConfig, BedrockPlayConfig};
use mcrust_bridge::{spawn_outbound_pump, BridgeRouter};
use mcrust_core::{default_spawn, GameConfig, GameHandle};
use mcrust_java::{JavaPlayConfig, JavaServerConfig, JavaStatusConfig, ServerKeys};
use tracing::info;
use tracing_subscriber::EnvFilter;

use config::ServerConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("mcrust=info".parse().unwrap()))
        .init();

    let conf_path = env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("conf.txt"));

    let cfg = match ServerConfig::load(&conf_path) {
        Ok(c) => c,
        Err(e) => {
            tracing::warn!(path = %conf_path.display(), error = %e, "using defaults");
            ServerConfig::default()
        }
    };

    let game = GameHandle::new(GameConfig {
        spawn: default_spawn(),
        tick_ms: 1000 / cfg.target_tps.max(1) as u64,
    });
    let router = BridgeRouter::new(&game);
    spawn_outbound_pump(game, router.clone());

    let keys = Arc::new(
        ServerKeys::generate().map_err(|e| format!("rsa keys: {e}"))?,
    );
    let http = reqwest::Client::new();

    let java_bind = SocketAddr::from((Ipv4Addr::UNSPECIFIED, cfg.server_port));
    let java_cfg = Arc::new(JavaServerConfig {
        status: JavaStatusConfig {
            motd: cfg.motd.clone(),
            max_players: cfg.max_players,
            online_players: 0,
        },
        play: JavaPlayConfig {
            online_mode: cfg.online_mode,
            prevent_proxy_connections: false,
        },
    });

    let router_j = router.clone();
    let java = tokio::spawn(async move {
        if let Err(e) =
            mcrust_java::run_java_listener(java_bind, java_cfg, router_j, keys, http).await
        {
            tracing::error!(error = %e, "java listener exited");
        }
    });

    let bedrock = if cfg.enable_bedrock {
        let bedrock_bind = SocketAddr::from((Ipv4Addr::UNSPECIFIED, cfg.bedrock_port));
        let ping_cfg = Arc::new(BedrockPingConfig {
            motd_line: cfg.motd.clone(),
            server_guid: 0x2D05F9D2E7BC4A1Fu64,
            max_players: cfg.max_players,
            online_players: 0,
        });
        let play_cfg = Arc::new(BedrockPlayConfig {
            online_mode: cfg.bedrock_online_mode,
            motd: cfg.motd.clone(),
        });
        let router_b = router.clone();
        Some(tokio::spawn(async move {
            if let Err(e) =
                mcrust_bedrock::run_bedrock_hybrid(bedrock_bind, ping_cfg, play_cfg, router_b).await
            {
                tracing::error!(error = %e, "bedrock listener exited");
            }
        }))
    } else {
        None
    };

    info!(
        name = %cfg.server_name,
        java_port = cfg.server_port,
        bedrock_port = cfg.bedrock_port,
        online_mode = cfg.online_mode,
        bedrock_online_mode = cfg.bedrock_online_mode,
        "mcrust: java login+play, core tick, bedrock ping (P6 RakNet session WIP)"
    );

    tokio::signal::ctrl_c().await?;
    info!("shutting down");
    java.abort();
    if let Some(b) = bedrock {
        b.abort();
    }
    Ok(())
}