use core::panic;
use std::{
    fs,
    io::{BufRead, BufReader},
    net::{TcpListener, TcpStream},
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
}

fn get_contents() {
    // get contents of logfile

    // i had a super dope one liner to do this whole function
    // but the borrow checker noped it
    // R.I.P. Dope One Liner: 1/20/23 17:51
    let i_hate_the_borrow_checker_it_made_me_make_this_owned_contents_binding = fs::read_to_string(
        xdg::BaseDirectories::with_prefix("processlogger")
            .expect("failed to create basedirectories struct")
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

    let test: Vec<(&str, &str)> = fs::read_to_string(
        xdg::BaseDirectories::with_prefix("processlogger")
            .expect("failed to create basedirectories struct")
            .get_cache_file("processlogfile"))
        .expect("Failed to read logfile")
        .lines() // iter over eache line
        .map(|line| line.split_once(",")) // split at first comma
        .collect::<Vec<Option<(&str, &str)>>>()
        .iter()
        .map(|opt| opt.unwrap())
        .collect();
    dbg!(test);
}
