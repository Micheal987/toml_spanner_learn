use std::fs;

use super::service_conf::ServiceConfig;
use toml_spanner::{ Arena, Context, Failed, FromToml, Item, Toml };
#[derive(Debug, Toml)]
#[toml(ToToml)]
pub struct Config {
    pub service: ServiceConfig,
}

impl<'de> FromToml<'de> for Config {
    fn from_toml(ctx: &mut Context<'de>, item: &Item<'de>) -> Result<Self, Failed> {
        let mut th = item.table_helper(ctx)?;
        let config = Config {
            service: th.optional("service").unwrap_or(ServiceConfig {
                host: Some("0.0.0.0".to_string()),
                port: Some(0),
                env: Some("default".to_string()),
            }),
        };
        th.require_empty()?;
        Ok(config)
    }
}
impl Config {
    pub fn new() -> Self {
        let doc_str = fs::read_to_string("setting.toml").expect("File Not Find");
        let arena = Arena::new();
        let mut doc = toml_spanner::parse(&doc_str, &arena).expect("toml_spanner parse Is Err");
        let config = doc.to::<Config>().expect("doc to Err");
        config
    }
}
