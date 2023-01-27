use crate::config::DayConfig;
use chrono::{Datelike, Weekday::*};
use pl_shared::{DayLog, MinLog, RonLog, LOGFILE_NAME};
use std::{fs, collections::HashSet};
use sysinfo::{ProcessExt, System, SystemExt};

const MINUTE: std::time::Duration = std::time::Duration::from_secs(60);
#[cfg(debug_assertions)]
const TEN_SECONDS: std::time::Duration = std::time::Duration::from_secs(60);

mod config;

fn main() {
    // get xdg config dir
    let xdg = pl_shared::init();

    let cfg = config::load_config(&xdg.get_config_file("config.ron"));

    let logfile_path = &xdg
        .place_cache_file(LOGFILE_NAME)
        .expect("Could not create leading dirs");

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
            Sun => &cfg.sunday,
        };

        /*
        // loop until daycfg.start
        while &format!("{}", chrono::Local::now().format("%H:%M")) != &daycfg.start {
            std::thread::sleep(MINUTE);
        } */

        // create new log day
        let mut ronlog: RonLog = ron::from_str(&match fs::read_to_string(logfile_path) {
            Ok(s) => s,
            Err(e) => {
                dbg!(e);
                // create empty log to write
                let emptylog =
                    ron::to_string(&RonLog::default()).expect("Failed to create empty ron");
                fs::write(logfile_path, &emptylog).expect("Failed to write/create logfile");
                emptylog
            }
        })
        .expect("failed to convert to ron");
        ronlog.days_logs.push(DayLog {
            date: chrono::offset::Local::now().format("%d/%m/%h").to_string(),
            logs: Vec::new(),
        });
        fs::write(
            logfile_path,
            ron::to_string(&ronlog).expect("Failed toead ron into string"),
        )
        .expect("Failed to write logfile");
        // this struct can in theory be infinitely large so we should drop it
        drop(ronlog);

        // this represents 1 minute between checks
        let mut iters = 0;
        loop {
            // get the time for this loop
            let time = format!("{}", chrono::Local::now().format("%H:%M"));

            // get processes
            let mut sys = System::new_all();
            sys.refresh_all();
            let mut procs: HashSet<String> = HashSet::new();
            for proc in sys.processes().iter().map(|(_pid, p)| (p.name())) {
                procs.insert(format!("{proc}"));
            }

            // get logfile
            let mut ronlog: RonLog =
                ron::from_str(&fs::read_to_string(logfile_path).expect("failed to read logfile"))
                    .expect("Could not read logfile into ron");
            let len = ronlog.days_logs.len() - 1;
            ronlog.days_logs[len].logs.push(MinLog {
                // jank method to not move time
                time: format!("{}", &time),
                procs: procs,
            });

            fs::write(
                logfile_path,
                ron::to_string(&ronlog).expect("Failed to convert ron to string"),
            )
            .expect("Failed to write log");

            if daycfg.end == time {
                break;
            }
            iters += 1;
            println!("Done w/ iter {iters}");
            std::thread::sleep(TEN_SECONDS);
        }
    }
}
