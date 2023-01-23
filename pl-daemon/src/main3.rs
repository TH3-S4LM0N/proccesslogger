use crate::config::DayConfig;
use chrono::Datelike;
use std::{
    fs::{File, OpenOptions, self},
    io::Write,
};
use sysinfo::{ProcessExt, System, SystemExt, Process};
use pl_shared::{RonLog, DayLog, MinLog};

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
        pl_shared::init();

    let cfg = config::load_config(&xdg.get_config_file("config.ron"));

    // this program lasts through days
    // this loop represents the day cycle,
    // 1 iteration should last a day
    loop {
        // figure out what day it is and set
        // the config accordingly
        let day = chrono::Local::now().date_naive().weekday();
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
        



        let logfile_path = &xdg
            .place_cache_file("processlogfile")
            .expect("Could not create parent dirs for logfile");

        // this represents 1 minute inbetween checks
        loop {
            // get the time for this loop
            let time = format!("{}", chrono::Local::now().format("%H:%M"));

            // get processes
            let mut system = {
                let s = System::new_all(); s.refresh_all();
                let mut procs: Vec<&str> = Vec::new();
                for proc in s.processes().iter().map(|(_pid, p)| (p.name())) {
                    procs.push(proc);
                };
                procs
            };

            let mut ronlog: RonLog = ron::from_str(&fs::read_to_string(logfile_path).expect("Failed to read logfile")).expect("Failed to convert to ron");

            /*
            //let to_write = ron::to_string();
            let to_write = format!("{},{:?}", chrono::Local::now().format("%H:%M"), procs); //.as_bytes();

            // get file
            println!("Create file");
            salmon::fs::append(logfile_path, &to_write).expect("I am anger");
            */
            std::thread::sleep(TEN_SECONDS);
        }
    }
}
