use std::net::SocketAddr;
use std::sync::Arc;

use mcrust_bridge::BridgeRouter;
use rust_raknet::RaknetListener;
use tracing::info;

use crate::session::handle_raknet_session;
use crate::config::BedrockPlayConfig;

pub async fn run_bedrock_server(
    bind: SocketAddr,
    cfg: Arc<BedrockPlayConfig>,
    router: BridgeRouter,
) -> Result<(), String> {
    let std_sock =
        std::net::UdpSocket::bind(bind).map_err(|e| format!("udp bind: {e}"))?;
    let mut listener = RaknetListener::from_std(std_sock)
        .await
        .map_err(|e| format!("raknet bind: {e:?}"))?;
    let motd_name = cfg.motd.replace(';', ",");
    listener
        .set_motd(
            &motd_name,
            0,
            "671",
            "1.21.50",
            "Survival",
            bind.port(),
        )
        .await;
    listener.listen().await;
    info!(%bind, "bedrock raknet listener ready (P6 login/play)");
    loop {
        let socket = listener
            .accept()
            .await
            .map_err(|e| format!("raknet accept: {e:?}"))?;
        let router = router.clone();
        let cfg = cfg.clone();
        tokio::spawn(async move {
            handle_raknet_session(socket, router, cfg).await;
        });
    }
}