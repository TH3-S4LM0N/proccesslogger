use std::{
    collections::HashMap,
    fs::{create_dir, read_to_string, File},
    hash::Hash,
    io::{Read, Write},
};

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
    // parse arg
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 1 {
        dbg!(args);
    }

    // get xdg config dir
    // the intention is that its run as a userspace systemd module
    let xdg =
        xdg::BaseDirectories::with_prefix("processlogger").expect("Failed to create xdg dirs");

    // get config
    let cfg = config::load_config(&xdg.get_config_file("config.toml"));

    // this program lasts through days etc
    'mainloop: loop {
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

        let mut file = File::open(LOGFILE).expect("Failed to open file");
        let mut contents: String = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read file");
        file.write_all(format!("{}\n---{}---", contents, chrono::Local::now()).as_bytes())
            .expect("Failed to write file.");

        // create system
        let mut system = System::new_all();

        let mut last_procs: &HashMap<Pid, Process>;
        // until the end
        while &format!("{}", chrono::Local::now().format("%H:%M")) != &daycfg.end {
            // get processes
            system.refresh_all();
            let mut procs = &mut system.processes();
            

            // get diff between these 2
            let diff = diff::diff_hashmap(last_procs, procs);

            // write diff
            let file = File::open(LOGFILE).expect("Failed to create file");
            let mut contents: String;
            file.read_to_string(&mut contents)
                .expect("Failed to read file");
            file.write_all(
                format!("{} -- {:?}", chrono::Local::now().format("%H:%M"), diff).as_bytes(),
            )
            .expect("Failed to write file");

            last_procs = procs;

            std::thread::sleep(BETWEEN_CHECKS);
        }
    }
}
