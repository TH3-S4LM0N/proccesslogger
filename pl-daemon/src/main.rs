use std::{fs::File, io::Write};
use sysinfo::{System, SystemExt, ProcessExt};
use crate::config::DayConfig;
use chrono::Datelike;

mod config;

//const HOUR: std::time::Duration = std::time::Duration::from_secs(3600);
const MINUTE: std::time::Duration = std::time::Duration::from_secs(60);

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

    let cfg = config::load_config(&xdg.get_config_file("config.toml"));

    // this program lasts through days
    // this loop represents the day cycle,
    // 1 iteration should last a day
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

        // loop until daycfg.start
        while &format!("{}", chrono::Local::now().format("%H:%M")) != &daycfg.start {
            std::thread::sleep(MINUTE);
        }

        /*
         * Format of logfile
         * ```
         * <time in a "Hour:Minute" format>,<all processes>
         * ```
         */

        let logfile_path = &xdg
            .place_cache_file("processlogfile")
            .expect("Could not create parent dirs for logfile");

        // this represents 1 minute inbetween checks
        loop {
            // get processes
            let mut system = System::new_all();
            system.refresh_all();
            let procs = system.processes().iter().map(|(pid, p)| {
                (pid, (p.name()))
            });

            
            let to_write = format!("{},{:?}", chrono::Local::now().format("%H:%M"), procs); //.as_bytes();

            // get file
            let filer = File::open(logfile_path);
            let mut file = match filer {
                Ok(f) => f,
                Err(e) => {
                    dbg!(e);
                    let file = File::create(logfile_path).expect("Failed to create logfile");
                    file
                }
            };
            file.write_all(to_write.as_bytes()).expect("Failed to write entire buffer");
        }
    }
}