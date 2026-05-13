use toml_spanner::{Toml};
#[derive(Debug, Toml)]
#[toml(FromToml, ToToml)] // By default only `FromToml` is derived.
pub struct ServiceConfig {
    pub host: Option<String>,
    pub port: Option<u32>,
    pub env: Option<String>,
}
impl ServiceConfig {
    pub fn host(&self) -> &str {
        self.host.as_deref().unwrap_or("0.0.0.0")
    }
    pub fn port(&self) -> u32 {
        self.port.unwrap_or(0)
    }
    pub fn env(&self) -> &str {
        self.env.as_deref().unwrap_or("default")
    }
}
