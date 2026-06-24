use thiserror::Error;

#[derive(Debug, Error)]
pub enum WireError {
    #[error("unexpected end of buffer")]
    Eof,
    #[error("varint too long")]
    VarIntTooLong,
    #[error("string exceeds maximum length ({max}): got {len}")]
    StringTooLong { max: usize, len: usize },
    #[error("invalid UTF-8 in string")]
    Utf8(#[from] std::str::Utf8Error),
    #[error("invalid UTF-8 in string (from_utf8)")]
    FromUtf8(#[from] std::string::FromUtf8Error),
}

pub type WireResult<T> = Result<T, WireError>;
