use sysinfo::{System, SystemExt};

mod config;
mod diff;

use crate::{config as cconfig, diff as cdiff};

const HOUR: std::time::Duration = std::time::Duration::from_secs(3600);
const MINUTE: std::time::Duration = std::time::Duration::from_secs(60);
const BETWEEN_CHECKS: std::time::Duration = std::time::Duration::from_secs(60);
const LOGFILE: &str = "~/.processlogfile";

fn main() {
    // get config
    let cfg = config::load_config();

    // get xdg config dir
    // the intention is that its run as a userspace systemd module
    let xdg = xdg::BaseDirectories::with_prefix("processlogger").expect("Failed to create xdg dirs");

}
