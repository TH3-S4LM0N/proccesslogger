// if this actually becomes useful i might make a crate of it

/*
 * TODO
 * - config in something readable, toml prolly
 *
 *
 */

use core::panic;
use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
};

use serde::{Deserialize};
use sysinfo::{Pid, Process, ProcessExt, System, SystemExt};

fn main() {
    let now = format!("{}", chrono::Local::now().format("%H:%M"));
    let pattern430 = regex::Regex::new(r#"^\d{2}:30"#).unwrap();

    // loop until it is `**:30`
    loop {
        if pattern430.is_match(&now) {
            if &now == "22:30" {
                break;
            } else {
                std::thread::sleep(HOUR);
            }
        } else {
            std::thread::sleep(MINUTE);
        }
    }

    println!("{:?}", chrono::Local::now());

    let mut system = System::new_all();

    /*
     * 'event' loop steps:
     * - get all processes
     * - diff against start processes
     * - diff against procs from last iteration
     * - format somehow
     * - write to file
     * - get new current time
     */

    let mut last_iter_procs: &HashMap<Pid, Process> = &HashMap::new(); 
    
    loop {

    }
    
    unreachable!();
    loop {
        
        // get new process list
        let mut procs = sys.processes();

        // get diff
        let diff = diff_hashmap(procs, last_iter_procs);
        println!("diff for dbg");
        dbg!(&diff);

        // only log if there is a change
        if &diff != &Vec::<String>::new() {
            let msg = format!(
                "At {} new process(es): {:?}",
                chrono::Local::now().format("%H:%M"),
                &diff
            );

            // write log
            let mut file = File::open(LOGFILE).expect("Failed to open logfile");
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .expect("Failed to read file");
            contents += &format!("\n{}", msg);
            file.write_all(contents.as_bytes())
                .expect("Failed to write file");
        }
        let ownd = procs;
        last_iter_procs = procs.to_owned();
    }
}

fn diff_hashmap(map1: &HashMap<Pid, Process>, map2: &HashMap<Pid, Process>) -> Vec<String> {
    // diff map to return
    let mut rv: Vec<String> = Vec::new();
    for (pid1, process1) in map1 {
        for (pid2, process2) in map2 {
            if &pid1 == &pid2 {
                rv.push(process2.name().to_string());
            }
        }
    }

    return rv;
}

