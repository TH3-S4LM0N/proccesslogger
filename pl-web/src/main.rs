use core::panic;
use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream}, ops::ControlFlow,
};
use serde::{Serialize, Deserialize};

mod logformat;

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
    let contents = logformat::get_contents();
    let length = contents.len();
    let response = 
        format!("{}\r\nContent-Lenth: {}\r\n\r\n{}", status_line, length, contents);
    stream.write_all(response.as_bytes()).expect("Failed to write entire buffer");
}



pub fn diff_vec<P: std::fmt::Debug>(vector: Vec<P>) {

}
