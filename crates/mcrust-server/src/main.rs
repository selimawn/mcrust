mod config;

use std::env;
use std::path::PathBuf;

use tracing::info;
use tracing_subscriber::EnvFilter;

use config::ServerConfig;

fn main() {
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

    info!(
        name = %cfg.server_name,
        java_port = cfg.server_port,
        bedrock = cfg.enable_bedrock,
        bedrock_port = cfg.bedrock_port,
        online_mode = cfg.online_mode,
        bedrock_online_mode = cfg.bedrock_online_mode,
        "mcrust starting (network listeners not yet implemented)"
    );

    let _ = mcrust_wire::var_int::var_int_len(0);
    let _ = mcrust_protocol::Platform::Java;
}
