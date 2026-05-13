use std::fs;

use super::service_conf::ServiceConfig;
use toml_spanner::{ Arena, Toml };
#[derive(Debug, Toml)]
#[toml(FromToml, ToToml)] // By default only `FromToml` is derived.
pub struct Config {
    pub service: ServiceConfig,
}
impl Config {
    pub fn new() -> Self {
        let doc_str = fs::read_to_string("setting.toml").expect("toml File Not Find");
        let arena = Arena::new();
        let mut doc = toml_spanner::parse(&doc_str, &arena).expect("toml_spanner Parse Err");
        let config = doc.to::<Config>().expect("Doc Is Err");
        config
    }
}
