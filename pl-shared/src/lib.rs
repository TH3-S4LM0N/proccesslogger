use std::collections::HashSet;

use serde::{Deserialize, Serialize};

pub const LOGFILE_NAME: &str = "processlogfile";

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RonLog {
    pub days_logs: Vec<DayLog>
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DayLog {
    // date: "01/22/2023"
    pub date: String,
    pub logs: Vec<MinLog>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MinLog {
    // time: "22:30"
    pub time: String,
    pub procs: HashSet<String>
}

pub fn init() -> xdg::BaseDirectories {
    xdg::BaseDirectories::with_prefix("processlogger")
        .expect("Failed to create basedirectories struct")
}

