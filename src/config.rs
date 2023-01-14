use serde::Deserialize;
use std::fs::File;
use crate::CONFIG_PATH;

pub fn load_config() -> Config {
    let cfgfiler = std::fs::read_to_string(CONFIG_PATH);
    let cfgfile = match cfgfiler {
        Ok(f) => f,
        Err(e) => panic!("{:?}", e)
    };

    let cfg: Config = toml::from_str(&cfgfile).expect("Failed to create toml config");

    return cfg;
}

pub fn create_config() {
    //let cfgfile = File::create(CONFIG_PATH).expect("Failed to create cfgfile");

}

#[derive(Deserialize, Debug)]
pub struct Config {
    monday: DayConfig,
    tuesday: DayConfig,
    wednsday: DayConfig,
    thursday: DayConfig,
    friday: DayConfig,
    saturday: DayConfig,
    sunday: DayConfig    
}
#[derive(Deserialize, Debug)]
struct DayConfig {
    start: String,
    length: String
}