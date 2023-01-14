use sysinfo::{System, SystemExt};

mod config;
mod diff;
mod clone4sys; // lol hot garbage code

const HOUR: std::time::Duration = std::time::Duration::from_secs(3600);
const MINUTE: std::time::Duration = std::time::Duration::from_secs(60);
const BETWEEN_CHECKS: std::time::Duration = std::time::Duration::from_secs(60);
const LOGFILE: &str = "~/.processlogfile";
const CONFIG_PATH: &str = "/etc/processlogger.conf.toml";

impl Clone for System {}

fn main() {
    let system: System = System::new_all(); 

    // stuff


    // loop
    loop {
        let sys = system.clone();
    }
}