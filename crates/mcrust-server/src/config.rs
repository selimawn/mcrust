use std::fs;
use std::path::Path;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid line {line}: {message}")]
    InvalidLine { line: usize, message: String },
}

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub server_name: String,
    pub motd: String,
    pub max_players: u32,
    pub server_port: u16,
    pub bedrock_port: u16,
    pub online_mode: bool,
    pub bedrock_online_mode: bool,
    pub enable_bedrock: bool,
    pub level_name: String,
    pub target_tps: u32,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            server_name: "mcrust".into(),
            motd: "A mcrust server".into(),
            max_players: 20,
            server_port: 25565,
            bedrock_port: 19132,
            online_mode: true,
            bedrock_online_mode: true,
            enable_bedrock: true,
            level_name: "world".into(),
            target_tps: 20,
        }
    }
}

impl ServerConfig {
    pub fn load(path: &Path) -> Result<Self, ConfigError> {
        let content = fs::read_to_string(path)?;
        Self::parse(&content)
    }

    pub fn parse(content: &str) -> Result<Self, ConfigError> {
        let mut cfg = Self::default();
        for (idx, line) in content.lines().enumerate() {
            let line_num = idx + 1;
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let Some((key, value)) = line.split_once('=') else {
                return Err(ConfigError::InvalidLine {
                    line: line_num,
                    message: "expected key=value".into(),
                });
            };
            let key = key.trim();
            let value = value.trim();
            match key {
                "server-name" => cfg.server_name = value.to_string(),
                "motd" => cfg.motd = value.to_string(),
                "max-players" => cfg.max_players = parse_u32(value, line_num)?,
                "server-port" => cfg.server_port = parse_u16(value, line_num)?,
                "bedrock-port" => cfg.bedrock_port = parse_u16(value, line_num)?,
                "online-mode" => cfg.online_mode = parse_bool(value, line_num)?,
                "bedrock-online-mode" => cfg.bedrock_online_mode = parse_bool(value, line_num)?,
                "enable-bedrock" => cfg.enable_bedrock = parse_bool(value, line_num)?,
                "level-name" => cfg.level_name = value.to_string(),
                "target-tps" => cfg.target_tps = parse_u32(value, line_num)?,
                other => {
                    tracing::debug!(key = other, "ignored unknown conf.txt key");
                }
            }
        }
        Ok(cfg)
    }
}

fn parse_bool(s: &str, line: usize) -> Result<bool, ConfigError> {
    match s {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(ConfigError::InvalidLine {
            line,
            message: format!("expected true or false, got {s}"),
        }),
    }
}

fn parse_u16(s: &str, line: usize) -> Result<u16, ConfigError> {
    s.parse().map_err(|_| ConfigError::InvalidLine {
        line,
        message: format!("invalid u16: {s}"),
    })
}

fn parse_u32(s: &str, line: usize) -> Result<u32, ConfigError> {
    s.parse().map_err(|_| ConfigError::InvalidLine {
        line,
        message: format!("invalid u32: {s}"),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_example_keys() {
        let s = r#"
online-mode=false
bedrock-online-mode=true
server-port=25566
"#;
        let c = ServerConfig::parse(s).unwrap();
        assert!(!c.online_mode);
        assert!(c.bedrock_online_mode);
        assert_eq!(c.server_port, 25566);
    }
}
