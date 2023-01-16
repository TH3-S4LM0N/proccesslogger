use std::{collections::HashMap, fs::{File, create_dir, read_to_string}, hash::Hash, io::{Write, Read}};

use chrono::Datelike;
use config::DayConfig;
use sysinfo::{Pid, Process, System, SystemExt};

mod config;
mod diff;
//mod time;

const HOUR: std::time::Duration = std::time::Duration::from_secs(3600);
const MINUTE: std::time::Duration = std::time::Duration::from_secs(60);
const BETWEEN_CHECKS: std::time::Duration = std::time::Duration::from_secs(60);
const LOGFILE: &str = "~/.processlogfile";

fn main() {
    let now = chrono::Local::now();
    dbg!(now);
    let nowutc = chrono::Utc::now();
    dbg!(nowutc);
    panic!();

    // parse arg
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 1 {}

    // get xdg config dir
    // the intention is that its run as a userspace systemd module
    let xdg =
        xdg::BaseDirectories::with_prefix("processlogger").expect("Failed to create xdg dirs");

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
            _ => unreachable!("{} is invalid!", day.as_str()),
        };

        // loop until it is daycfg.start
        while &format!("{}", chrono::Local::now().format("%H:%M")) != &daycfg.start {
            std::thread::sleep(MINUTE);
        }

        let file = File::open(LOGFILE).expect("Failed to open file");
        let mut contents: String;
        file.read_to_string(&mut contents).expect("Failed to read file");
        file.write_all(format!("{}\n---{}---", contents, chrono::Local::now()).as_bytes()).expect("Failed to write file.");


        let mut last_procs: &HashMap<Pid, Process>;
        // until the end
        while &format!("{}", chrono::Local::now().format("%H:%M")) != &daycfg.end {
            let mut system = System::new_all();
            system.refresh_all();
            let procs = system.processes();

            let diff = diff::diff_hashmap(last_procs, procs);

            let file = File::open(LOGFILE).expect("Failed to create file");
        }
    }
}
