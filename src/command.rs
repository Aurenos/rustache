#![allow(unused_variables)]

use std::collections::HashMap;
use std::str::FromStr;

type Database = HashMap<String, String>;
type CmdOutput = Result<String, CmdError>;
type ErrorMsg = Option<String>;

#[derive(Debug, PartialEq)]
pub enum Cmd {
    Ping,
    Echo { output: String },
    Set { key: String, value: String },
    Get { key: String },
    Del { key: String },
}

#[derive(Debug, PartialEq)]
pub enum CmdError {
    UnknownCommandError(ErrorMsg),
    InvalidCommandError(ErrorMsg),
    InvalidArgumentError(ErrorMsg),
    DatabaseError(ErrorMsg),
}

fn parse_args(args: Option<String>) -> Vec<String> {
    args.clone()
        .get_or_insert("".to_string())
        .split_ascii_whitespace()
        .map(|s| s.to_string())
        .collect()
}

impl Cmd {
    pub fn handle(self, db: &mut Database) -> CmdOutput {
        use Cmd::*; // let's us use the Cmd invariants without prefixing them with `Self::`

        match self {
            Ping => Ok(String::from("PONG")),
            Echo { output } => Ok(output),
            Set { key, value } => Self::handle_set(key, value, db),
            Get { key } => Self::handle_get(key, db),
            Del { key } => Self::handle_del(key, db),
        }
    }

    fn handle_set(key: String, value: String, db: &mut Database) -> CmdOutput {
        db.insert(key.clone(), value.clone());
        Ok(format!("\"{}\":\"{}\"", key, value))
    }

    fn handle_get(key: String, db: &mut Database) -> CmdOutput {
        db.get(key.as_str())
            .ok_or_else(|| CmdError::DatabaseError(Some("No value for key".to_string())))
            .map(|s| s.to_string())
    }

    fn handle_del(key: String, db: &mut Database) -> CmdOutput {
        if db.remove(key.as_str()).is_some() {
            Ok(format!("Key \"{}\" deleted", key))
        } else {
            Err(CmdError::DatabaseError(Some(format!(
                "Key \"{}\" does not exist",
                key
            ))))
        }
    }
}

impl FromStr for Cmd {
    type Err = CmdError;

    fn from_str(input: &str) -> Result<Cmd, Self::Err> {
        use CmdError::InvalidArgumentError;

        let mut splitter = input.trim_end().splitn(2, ' ');
        let cmd_str = splitter.next().unwrap().to_uppercase();
        let args = parse_args(splitter.next().map(|s| s.to_string()));

        let cmd = match cmd_str.as_str() {
            "PING" => Cmd::Ping,
            "ECHO" => {
                if args.is_empty() {
                    return Err(InvalidArgumentError(Some("Nothing to echo".to_string())));
                } else {
                    Cmd::Echo {
                        output: args.join(" "),
                    }
                }
            }
            "SET" => {
                if args.is_empty() {
                    return Err(InvalidArgumentError(Some("No key specified".to_string())));
                } else if args.len() < 2 {
                    return Err(InvalidArgumentError(Some("No value specified".to_string())));
                } else {
                    Cmd::Set {
                        key: args[0].clone(),
                        value: args[1].clone(),
                    }
                }
            }
            "GET" => {
                if args.is_empty() {
                    return Err(InvalidArgumentError(Some("No key specified".to_string())));
                } else {
                    Cmd::Get {
                        key: args[0].clone(),
                    }
                }
            }
            "DEL" => {
                if args.is_empty() {
                    return Err(InvalidArgumentError(Some("No key specified".to_string())));
                } else {
                    Cmd::Del {
                        key: args[0].clone(),
                    }
                }
            }
            _ => {
                return Err(CmdError::UnknownCommandError(Some(cmd_str.to_string())));
            }
        };

        Ok(cmd)
    }
}

impl CmdError {
    fn get_full_msg(&self, name: &'static str, msg: &ErrorMsg) -> String {
        let mut full_msg = String::from(name);
        if let Some(m) = msg {
            full_msg.push_str(format!(": {}", m).as_str())
        }
        full_msg
    }
}

impl ToString for CmdError {
    fn to_string(&self) -> String {
        // TODO: Maybe look up how to do reflection to extract the symbol name
        match self {
            Self::UnknownCommandError(msg) => self.get_full_msg("UnknownCommandError", msg),
            Self::InvalidCommandError(msg) => self.get_full_msg("InvalidCommandError", msg),
            Self::InvalidArgumentError(msg) => self.get_full_msg("InvalidArgumentError", msg),
            Self::DatabaseError(msg) => self.get_full_msg("DatabaseError", msg),
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn str_to_cmd() {
    //     assert_eq!(Cmd::from_str("pInG"), Ok(Cmd::Ping));

    //     assert_eq!(
    //         Cmd::from_str("get schwifty"),
    //         Ok(Cmd::Get(Some("schwifty".to_string())))
    //     );

    //     assert!(matches!(
    //         Cmd::from_str("spiarmf slurmp"),
    //         Err(CmdError::UnknownCommandError(_))
    //     ));
    // }

    // #[test]
    // fn handle_echo_cmd() {
    //     let mut db = Database::new();

    //     let cmd = Cmd::Echo(None);
    //     assert!(matches!(
    //         cmd.handle(&mut db),
    //         Err(CmdError::InvalidArgumentError(_))
    //     ));

    //     assert!(matches!(
    //         Cmd::Echo(Some("ermahgerd dergs".to_string())).handle(&mut db),
    //         Ok(_)
    //     ));

    //     assert!(matches!(
    //         Cmd::Echo(Some("".to_string())).handle(&mut db),
    //         Err(CmdError::InvalidArgumentError(_))
    //     ));

    //     assert!(matches!(
    //         Cmd::Echo(Some("  \n".to_string())).handle(&mut db),
    //         Err(CmdError::InvalidArgumentError(_))
    //     ));

    //     assert_eq!(
    //         Cmd::Echo(Some("Slurm\r\n".to_string())).handle(&mut db),
    //         Ok("Slurm".to_string())
    //     );
    // }
}
