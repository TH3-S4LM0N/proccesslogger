use crate::config::DayConfig;
use chrono::Datelike;
use std::{
    fs::{File, OpenOptions},
    io::Write,
};
use sysinfo::{ProcessExt, System, SystemExt, Process};

mod config;

//const HOUR: std::time::Duration = std::time::Duration::from_secs(3600);
const MINUTE: std::time::Duration = std::time::Duration::from_secs(60);
const TEN_SECONDS: std::time::Duration = std::time::Duration::from_secs(10);

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

        /*
        // loop until daycfg.start
        while &format!("{}", chrono::Local::now().format("%H:%M")) != &daycfg.start {
            std::thread::sleep(MINUTE);
        } */

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
            println!("create system");
            let mut system = System::new_all();
            system.refresh_all();
            let mut procs: Vec<&str> = Vec::new();
            for proc in system.processes().iter().map(|(pid, p)| (p.name())) {
                procs.push(proc);
            };

            println!("let to_write");
            let to_write = format!("{},{:?}", chrono::Local::now().format("%H:%M"), procs); //.as_bytes();

            // get file
            println!("Create file");
            let mut file = OpenOptions::new()
                .append(true)
                .write(true)
                .create(true)
                .open(logfile_path)
                .expect("Failed to open logfile");

            println!("Write file");
            writeln!(file, "{to_write}").expect("Failed to write logfile");
            // file.write_all(to_write.as_bytes())
            //    .expect("Failed to write entire buffer");

            std::thread::sleep(TEN_SECONDS);
        }
    }
}
