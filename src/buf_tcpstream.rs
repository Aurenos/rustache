use std::io::prelude::*;
use std::io::{self, BufReader, BufWriter};
use std::net::TcpStream;

pub struct BufTcpStream {
    input: BufReader<TcpStream>,
    output: BufWriter<TcpStream>,
}

impl BufTcpStream {
    pub fn new(stream: TcpStream) -> io::Result<Self> {
        let input = BufReader::new(stream.try_clone()?);
        let output = BufWriter::new(stream);

        Ok(Self { input, output })
    }

    pub fn send_msg(&mut self, msg: String) {
        self.output.write_all(msg.as_bytes()).unwrap();
        self.output.flush().unwrap();
    }

    pub fn recv(&mut self) -> String {
        let mut buffer = String::new();
        self.input.read_line(&mut buffer).unwrap();
        buffer
    }
}
