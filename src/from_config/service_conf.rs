use toml_spanner::{ Context, Failed, FromToml, Item, Toml };
#[derive(Debug, Toml)]
#[toml(ToToml)]
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
impl<'de> FromToml<'de> for ServiceConfig {
    fn from_toml(ctx: &mut Context<'de>, item: &Item<'de>) -> Result<Self, Failed> {
        let mut th = item.table_helper(ctx)?;
        let config = ServiceConfig {
            host: th.optional("host").unwrap_or(None),
            port: th.optional("port").unwrap_or(None),
            env: th.optional("env").unwrap_or(None),
        };
        th.require_empty()?;
        Ok(config)
    }
}
