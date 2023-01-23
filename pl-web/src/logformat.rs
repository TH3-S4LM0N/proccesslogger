use std::fs;

use serde::{Serialize, Deserialize};

pub fn get_contents() -> String {
    // get contents of logfile and format them
    // the problem here is that there is too much

    // i had a super dope one liner to do this whole function
    // but the borrow checker noped it
    // R.I.P. Dope One Liner: 1/20/23 17:51
    let i_hate_the_borrow_checker_it_made_me_make_this_owned_contents_binding = fs::read_to_string(
        pl_shared::init()
            .get_cache_file("processlogfile"),
    )
    .expect("Failed to read logfile");
    let contents_raw: Vec<(&str, &str)> =
        i_hate_the_borrow_checker_it_made_me_make_this_owned_contents_binding
            .lines() // iter over eache line
            .map(|line| line.split_once(",")) // split at first comma
            .collect::<Vec<Option<(&str, &str)>>>()
            .iter()
            .map(|opt| opt.unwrap())
            .collect();      
    
    // contents_raw is a vector containing tuples of the time
    // and a string containing a vector of strings w/
    // process names
    // contents_raw = [("17:17", "["proc1", "procs2"]")]

    // first vec is each day
    // nested vec is all the procs from said day
    // index 0 will always be empty
    let mut procs_undiffed: Vec<(&str, Vec<&str>)> = Vec::new(); //vec![("Empty Time", vec!["Empty Proc".to_string()])];
    let mut iters = 0;
    dbg!(&contents_raw);
    // vec being str of vec or procs
    for tup in contents_raw {
        //if iters == 0 {continue;}
        println!("iters: {iters}");
        let mut to_push: (&str, Vec<String>) = ("Time: Index 0 is empty", vec!["Proc1: Index 0 is empty".to_string()]);
        // set the time
        to_push.0 = tup.0;
        // format to ron
        let vecf = format!("(a:{})", tup.1);

        // this a vec of the processes finally
        let procs_vec: RonVec = ron::from_str(&vecf).expect("Failed to convert vec to ron");

        for proc in procs_vec.a {
            to_push.1.push(proc);
        }
        procs_undiffed.push(to_push);

        iters += 1;
    }
    
    // now we have all the stuff properly!!!!!!!!!!! omg
    // diff the vecs now

    // we can reuse this var
    iters = 1;
    let len = &procs_undiffed.len();
    let mut procs_diffed: Vec<(&str, Vec<String>)> = Vec::new();
    procs_diffed[0] = procs_undiffed[0];
    // its easier to just create a new var for this
    // we cant use for as i need acess to 2 different indexes
    loop {
        if iters == len - 1 {break;}
        let iters1 = iters + 1;

        if procs_undiffed[iters].1 == procs_undiffed[iters1].1 {

        }
    }

    /*loop {
        // check that we arent indexing out of the len
        if iters == len - 1 {break;}
        let iters1 = iters + 1;

        // check that they arent the same before we diff
        if procs_undiffed[iters].1 == procs_undiffed[iters1].1 {
            let _ = &procs_undiffed.remove(iters);
            iters += 1;
            continue;
        }
        // now we know they are different so we get the difference
        let mut idx1 = 0;
        let mut idx2 = 0;
        for proc1 in &mut procs_undiffed[iters].1 {
            
            for proc2 in &mut procs_undiffed[iters1].1 {
                
                if proc1 == proc2 {
                    procs_undiffed[iters].1.remove(idx1);
                }
                idx2 += 1;
            }
            idx1 += 1;
        }

        iters += 1;
    } */
    
    
    panic!();
    String::new()

}

#[derive(Serialize, Deserialize, Debug)]
struct RonVec {
    a: Vec<String>
}