use std::fs;
use chrono::{Datelike, Weekday::*};
use crate::config::DayConfig;
use pl_shared::{LOGFILE_NAME, RonLog};

mod config;

fn main() {


    // get xdg config dir
    let xdg = pl_shared::init();

    let cfg = config::load_config(&xdg.get_config_file("config.ron"));

    let logfile_path = &xdg.place_cache_file(LOGFILE_NAME).expect("Could not create leading dirs");

    // get log to keep appending too
    let mut ronlog: RonLog = ron::from_str(&fs::read_to_string(logfile_path).expect("Failed to read logfile_path")).expect("failed to convert to ron");

    // this lasts through multiple days
    // this loop represents the day cycle
    // 1 iter is 1 day long
    loop {
        // figure out what day it is and set config
        let daycfg: &DayConfig = match chrono::Local::now().date_naive().weekday() {
            Mon => &cfg.monday,
            Tue => &cfg.tuesday,
            Wed => &cfg.wednsday,
            Thu => &cfg.thursday,
            Fri => &cfg.friday,
            Sat => &cfg.saturday,
            Sun => &cfg.sunday
        };

        /*
        // loop until daycfg.start
        while &format!("{}", chrono::Local::now().format("%H:%M")) != &daycfg.start {
            std::thread::sleep(MINUTE);
        } */

        // this represents 1 minute between checks

        

    }
}