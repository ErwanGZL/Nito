use std::fs;
use toml;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Remote {
    address: String,
    port: String,
}

#[derive(Debug, Deserialize)]
pub struct World {
    x: u32,
    y: u32,
}
#[derive(Debug, Deserialize)]
pub struct Config {
    remote: Remote,
    world: World,
}

pub fn open_config() -> Config {
    let mut config_file = fs::read_to_string("config/config.toml").unwrap();
    let config: Config = toml::from_str(&config_file).unwrap();
    config
}