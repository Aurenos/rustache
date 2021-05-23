// #![allow(unused_imports)]
// #![allow(unused_variables)]

use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::str::FromStr;

use rustache::buf_tcpstream::BufTcpStream;
use rustache::command::Cmd;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();
    let mut database: HashMap<String, String> = HashMap::new();
    println!("Server started. Awaiting connections.");
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, &mut database);
    }
}

fn handle_connection(stream: TcpStream, db: &mut HashMap<String, String>) {
    let mut bufstream = BufTcpStream::new(stream).unwrap();
    let input = bufstream.recv();
    println!("Received: {}", input.trim_end());

    let command = Cmd::from_str(input.as_str());

    let output = match command {
        Ok(cmd) => cmd.handle(db),
        Err(invalid_cmd) => Err(format!("ERROR: Unknown command [{}]", invalid_cmd)),
    };

    let response = output.unwrap_or_else(|s| s);

    bufstream.send_msg(response);
}
