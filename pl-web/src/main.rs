use core::panic;
use pl_shared::{init, DayLog, MinLog, RonLog};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    ops::ControlFlow,
};

const PORT: &str = "127.0.0.1:8926";

fn main() {
    let listener = TcpListener::bind(PORT).expect(&format!("Couldnt open at {}", PORT));

    for stream in listener.incoming() {
        let stream = stream.expect("Failed to get stream");

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.expect("i have no idea"))
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 OK";
    let contents = get_contents();
    let length = contents.len();
    let response = format!(
        "{}\r\nContent-Lenth: {}\r\n\r\n{}",
        status_line, length, contents
    );
    stream
        .write_all(response.as_bytes())
        .expect("Failed to write entire buffer");
}

fn get_contents() -> String {
    let xdg = pl_shared::init();

    // now we have a usable owned log
    let ronlog: RonLog = ron::from_str(
        &fs::read_to_string(&xdg.get_cache_file("processlogfile")).expect("Failed to read logfile"),
    )
    .expect("Failed to read into ron");

    // now we have to get the diff between all of them
    // this is the new struct we will read from
    // when making the html
    let mut ronhtmlv = RonLog::default();
    // we index by this number
    let mut iters = 0;
    // in this loop we loop through all the days
    loop {
        // create a new day for this loop
        let _ = &ronhtmlv.days_logs.push(DayLog::default());
        // in this loop we loop through minutes
        let mut itersa = 0;
        loop {
            // create new log for thsi minute
            let _ = &ronhtmlv.days_logs[iters].logs.push(MinLog::default());

            // diff the stuff
            let diff1 = &ronlog.days_logs[iters].logs[itersa].procs
                - &ronlog.days_logs[iters].logs[itersa - 1].procs;
            let diff2 = &ronlog.days_logs[iters].logs[itersa - 1].procs
                - &ronlog.days_logs[iters].logs[itersa].procs;
            let diff: HashSet<&String> = diff1.union(&diff2).collect();

            ronhtmlv.days_logs[iters].logs[itersa].procs =
                diff.iter().map(|s| format!("{s}")).collect();

            itersa += 1;

            if &itersa == &ronlog.days_logs[iters].logs.len() {
                break;
            }
        }

        iters += 1;

        if &iters == &ronlog.days_logs.len() {
            break;
        }
    }

    dbg!(ronhtmlv);
    panic!();

    String::new()
}

pub fn diff_vec(a: Vec<String>, b: Vec<String>) {
    let mut rv: Vec<String> = Vec::new();

    for itema in a {
        for itemb in &b {
            if &itema != itemb {
                rv.push(itemb.to_string());
            }
        }
    }

    dbg!(rv);
    panic!();
}
