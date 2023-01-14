use std::collections::HashMap;

use sysinfo::{Pid, Process, ProcessExt};

pub fn diff_hashmap(
    map1: HashMap<Pid, Process>,
    map2: HashMap<Pid, Process>,
) -> Vec<String> {
    // diff map to return
    let mut rv: Vec<String> = Vec::new(); //vec![(String::new())];
    for (pid1, process1) in map1 {
        for (pid2, process2) in &map2 {
            if &pid1 == pid2 {
                rv.push(process2.name().to_string());
            }
        }
    }

    return rv;
}
