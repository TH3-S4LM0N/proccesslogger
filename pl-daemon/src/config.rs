use serde::{Deserialize, Serialize};
use std::{fs::{File, create_dir}, io::Write, path::PathBuf};

// we call this function in itself
pub fn load_config(config_path: &PathBuf) -> Config {
    let cfgfiler = std::fs::read_to_string(config_path);
    let cfgfile = match cfgfiler {
        Ok(f) => f,
        Err(e) => {
            println!("Failed to open config file: {:?}\nAttempting to recover", e);
            create_config(config_path);
            return load_config(config_path);
        }
    };

    let cfg: Config = toml::from_str(&cfgfile).expect("Failed to create toml config");

    return cfg;
}

pub fn create_config(config_path: &PathBuf) {
    create_dir(config_path.parent().unwrap()).expect("Failed to create config dir");
    let mut cfgfile = File::create(config_path).expect("Failed to create the config");
    cfgfile
        .write_all(
            toml::to_string(&Config::default())
                .expect("Failed to create toml str")
                .as_bytes(),
        )
        .expect("Failed to write toml");
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
