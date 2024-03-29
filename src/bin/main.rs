// #![allow(unused_imports)]
// #![allow(unused_variables)]

use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::str::FromStr;

use rustache::buf_tcpstream::BufTcpStream;
use rustache::command::{Cmd, CmdError};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();
    let mut database: HashMap<String, String> = HashMap::new();
    println!("Server started. Awaiting connections.");
    // TODO: make this handle multiple simulataneous connections eventually
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, &mut database);
    }
}

fn handle_connection(stream: TcpStream, db: &mut HashMap<String, String>) {
    let mut bufstream = BufTcpStream::new(stream).unwrap();
    let input = bufstream.recv();
    println!("Received: {}", input.trim_end());

    let output = Cmd::from_str(input.as_str()).and_then(|cmd| cmd.handle(db));

    bufstream.send_msg(prefix_output(output));
}

fn prefix_output(output: Result<String, CmdError>) -> String {
    let mut out = String::new();
    match output {
        Ok(msg) => {
            out.push('+');
            out.push_str(msg.as_str());
        }
        Err(err) => {
            out.push('-');
            out.push_str(err.to_string().as_str());
        }
    }
    out
}
