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

type Database = HashMap<String, String>;
type CmdOutput = Result<String, String>;

impl Cmd {
    pub fn handle(&self, db: &mut Database) -> CmdOutput {
        use Cmd::*; // let's us use the Cmd invariants without prefixing them with `Self::`

        match self {
            Ping => Ok(String::from("PONG")),
            Echo(args) => Self::handle_echo(args),
            Set(args) => Self::handle_set(args, db),
            Get(args) => Self::handle_get(args, db),

            Del(args) => Ok("DEL Command Received".to_string()),
        }
    }

    fn handle_echo(args: &Option<String>) -> CmdOutput {
        args.as_ref()
            .ok_or_else(|| "Nothing to echo".to_string())
            .map(|s| s.to_string())
    }

    fn handle_set(args: &Option<String>, db: &mut Database) -> CmdOutput {
        let mut tokens = args.as_ref().unwrap().splitn(2, ' ');
        let key = tokens.next().unwrap().to_string();
        let value = tokens.next().unwrap().to_string();
        db.insert(key.clone(), value.clone());
        Ok(format!("SET \"{}\":\"{}\"", key, value))
    }

    fn handle_get(args: &Option<String>, db: &mut Database) -> CmdOutput {
        let key = args.as_ref().unwrap().trim();
        let value = db.get(key).unwrap().to_string();
        Ok(value)
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
            _ => Err(format!("ERROR: Unknown command [{}]", command)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn str_to_cmd() {
        assert_eq!(Cmd::from_str("pInG"), Ok(Cmd::Ping));

        assert_eq!(
            Cmd::from_str("get schwifty"),
            Ok(Cmd::Get(Some("schwifty".to_string())))
        );

        assert_eq!(
            Cmd::from_str("spiarmf slurmp"),
            Err("ERROR: Unknown command [SPIARMF]".to_string())
        );
    }
}
