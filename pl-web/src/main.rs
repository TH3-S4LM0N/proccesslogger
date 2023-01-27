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

        // set date etc for this day
        ronhtmlv.days_logs[iters].date = format!("{}", &ronlog.days_logs[iters].date);
        // in this loop we loop through minutes
        let mut itersa = 0;
        loop {
            // create new log for thsi minute
            let _ = &ronhtmlv.days_logs[iters].logs.push(MinLog::default());

            // set time for min
            ronhtmlv.days_logs[iters].logs[itersa].time = format!("{}", &ronlog.days_logs[iters].logs[itersa].time);

            // diff the stuff
            let diff1 = &ronlog.days_logs[iters].logs[itersa].procs
                - &ronlog.days_logs[iters].logs[{
                    if itersa == 0 {
                        0
                    } else {
                        itersa - 1
                    }
                }]
                .procs;
            let diff2 = &ronlog.days_logs[iters].logs[{
                if itersa == 0 {
                    0
                } else {
                    itersa - 1
                }
            }]
            .procs
                - &ronlog.days_logs[iters].logs[itersa].procs;

            ronhtmlv.days_logs[iters].logs[itersa].procs =
                 diff1.union(&diff2).map(|s| format!("{s}")).collect();

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

    // ignore stuff
    let ignores = ron::from_str::<IgnoreFile>(&fs::read_to_string(xdg.get_config_file("ignorefile.ron")).expect("failed to read ignore file")).expect("failed to read to ron").ignores;


    // html that will be returned
    let mut html = format!("
<h1>Process Logger</h1>
");


    // reuse it again lol
    iters = 0;
    for day in &ronhtmlv.days_logs {
        html = format!("{}<h2>{}</h2>", html, day.date);
        let mut itersa = 0;
        for log in &ronhtmlv.days_logs[iters].logs {
            let mut procs_filtered: Vec<String> = Vec::new();
            for proc in &log.procs {
                for ignore in &ignores {
                    let regex = regex::Regex::new(ignore).expect("failed to create regex");
                    if !regex.is_match(proc) {
                        procs_filtered.push(format!("{proc}"));
                    }
                }
            }

            if !procs_filtered.is_empty() {
                html = format!("{}<h3>{}</h3>", html, log.time);
                for proc in procs_filtered {
                    html = format!("{}- {}<br>", html, proc);
                }
            }
            
            itersa += 1;
        }
        iters += 1;
    }
    
    return html;
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct IgnoreFile {
    ignores: Vec<String>
}