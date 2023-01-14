// if this actually becomes useful i might make a crate of it

/*
 * TODO
 * - config in something readable, toml prolly
 * 
 * 
 */
mod diff;
use core::panic;

use chrono::Duration;
use sysinfo::{SystemExt, System, ProcessExt};

const HOUR: std::time::Duration = 
    std::time::Duration::from_secs(3600);
const MINUTE: std::time::Duration =
    std::time::Duration::from_secs(60);

fn main() {

    let now = format!("{}", chrono::Local::now().format("%H:%M"));
    let pattern430 = regex::Regex::new(r#"^\d{2}:30"#).unwrap();
    
    // loop until it is `**:30`
    loop {

    }

    

    


    panic!();


    loop {
        // get running proccesses
        let mut sys = System::new_all();
        sys.refresh_all();

        let mut pcount = 0;
        for (pid, process) in sys.processes() {
            println!("{}", process.name());
            pcount += 1;
        }

        println!("{}", pcount);

        

        // diff them

        //format a `time: diff`

        // add it to a file

        // sleep however long
    }
}