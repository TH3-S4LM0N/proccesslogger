use serde::{Deserialize, Serialize};

pub const LOGFILE_NAME: &str = "processlogfile";

#[derive(Serialize, Deserialize, Debug)]
pub struct RonLog {
    pub days_logs: Vec<DayLog>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DayLog {
    pub date: String,
    pub logs: Vec<MinLog>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MinLog {
    pub time: String,
    pub procs: Vec<String>,
}

pub fn init() -> xdg::BaseDirectories {
    xdg::BaseDirectories::with_prefix("processlogger")
        .expect("Failed to create basedirectories struct")
}
