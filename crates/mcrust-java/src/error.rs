use thiserror::Error;

#[derive(Debug, Error)]
pub enum JavaError {
    #[error("wire: {0}")]
    Wire(#[from] mcrust_wire::WireError),
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("json: {0}")]
    Json(#[from] serde_json::Error),
    #[error("protocol: {0}")]
    Protocol(String),
}
