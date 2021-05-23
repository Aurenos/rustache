use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Cmd {
    Ping,
    Echo(Option<String>),
    Set(Option<String>),
    Get(Option<String>),
    Del(Option<String>),
}

impl Cmd {
    pub fn handle(&self, db: &mut HashMap<String, String>) -> Result<String, String> {
        // use Cmd::*; // You can use this if you want to remove the Self:: prefixing from below

        match self {
            Self::Ping => Ok(String::from("PONG")),
            Self::Echo(args) => match args {
                Some(string) => Ok(string.to_string()),
                None => Err("ERROR: Nothing to echo".to_string()),
            },

            Self::Set(args) => {
                let mut tokens = args.as_ref().unwrap().splitn(2, ' ');
                let key = tokens.next().unwrap().to_string();
                let value = tokens.next().unwrap().to_string();
                db.insert(key.clone(), value.clone());
                Ok(format!("SET \"{}\":\"{}\"", key, value))
            }

            Self::Get(args) => {
                let key = args.as_ref().unwrap().trim();
                let value = db.get(key).unwrap().to_string();
                Ok(value)
            }

            Self::Del(args) => Ok("DEL Command Received".to_string()),
        }
    }
}

impl FromStr for Cmd {
    type Err = String;

    fn from_str(input: &str) -> Result<Cmd, Self::Err> {
        let mut splitter = input.trim_end().splitn(2, ' ');
        let command = splitter.next().unwrap().to_uppercase();
        let args = splitter.next().map(|s| s.to_string());

        match command.as_str() {
            "PING" => Ok(Cmd::Ping),
            "ECHO" => Ok(Cmd::Echo(args)),
            "SET" => Ok(Cmd::Set(args)),
            "GET" => Ok(Cmd::Get(args)),
            "DEL" => Ok(Cmd::Del(args)),
            _ => Err(command.to_string()),
        }
    }
}
