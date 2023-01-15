use chrono::Datelike;
use config::DayConfig;

mod config;
mod diff;
mod dayloops;

const HOUR: std::time::Duration = std::time::Duration::from_secs(3600);
const MINUTE: std::time::Duration = std::time::Duration::from_secs(60);
const BETWEEN_CHECKS: std::time::Duration = std::time::Duration::from_secs(60);
const LOGFILE: &str = "~/.processlogfile";

fn main() {
    // parse arg
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 1 {
        
    }

    // get xdg config dir
    // the intention is that its run as a userspace systemd module
    let xdg = xdg::BaseDirectories::with_prefix("processlogger").expect("Failed to create xdg dirs");

    // get config
    let cfg = config::load_config(&xdg.get_config_file("config.toml"));

    // this program lasts through days etc
    loop {
        // figure out what day it is and set
        // the config accordingly
        let day = format!("{}", chrono::Local::now().date_naive().weekday());
        let daycfg: &DayConfig = match day.as_str() {
            "Mon" => &cfg.monday,
            "Tue" => &cfg.tuesday,
            "Wed" => &cfg.wednsday,
            "Thu" => &cfg.thursday,
            "Fri" => &cfg.friday,
            "Sat" => &cfg.saturday,
            "Sun" => &cfg.sunday,
            _ => unreachable!("{} is invalid!", day.as_str())
        };

        // loop until it is daycfg.start
        loop {
            println!("a{}a", daycfg.start);
        }
    }
}
