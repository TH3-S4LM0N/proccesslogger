use serde::{Deserialize, Serialize};
use std::{fs::read_to_string, path::PathBuf};

// we call this function in itself
pub fn load_config(config_path: &PathBuf) -> Config {
    ron::from_str(&read_to_string(config_path).expect("Failed to read config file, does it exist?"))
        .expect("Failed to create ron config")
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Config {
    pub monday: DayConfig,
    pub tuesday: DayConfig,
    pub wednsday: DayConfig,
    pub thursday: DayConfig,
    pub friday: DayConfig,
    pub saturday: DayConfig,
    pub sunday: DayConfig,
}
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct DayConfig {
    pub start: String,
    pub end: String,
}
