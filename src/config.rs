use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct Config {
    pub server: Server,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct Server {
    pub ip: String,
    pub port: u16,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            ip: "0.0.0.0".to_string(),
            port: 8080,
        }
    }
}
