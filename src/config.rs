use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write};

pub fn load_config() -> Config {
    let cfgfiler = std::fs::read_to_string(CONFIG_PATH);
    let cfgfile = match cfgfiler {
        Ok(f) => f,
        Err(e) => {
            create_config();
            return load_config();
        }
    };

    let cfg: Config = toml::from_str(&cfgfile).expect("Failed to create toml config");

    return cfg;
}

pub fn create_config() {
    let mut cfgfile = File::create(CONFIG_PATH).expect("Failed to create the config");
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
    monday: DayConfig,
    tuesday: DayConfig,
    wednsday: DayConfig,
    thursday: DayConfig,
    friday: DayConfig,
    saturday: DayConfig,
    sunday: DayConfig,
}
#[derive(Deserialize, Serialize, Debug, Default)]
struct DayConfig {
    start: String,
    length: String,
}
