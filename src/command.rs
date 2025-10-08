use crate::err_types::ParseError;

pub enum Command {
    Get { key: String },
    Set { key: String, val: String },
    Del { key: String },
}

///TODO String or &str?
pub fn parse(input: &str) -> Result<Command, ParseError> {
    //split into components (tokens)
    let parts = input.trim().split_whitespace().collect::<Vec<&str>>();
    if parts.is_empty() {
        return Err(ParseError::InvalidCommand("Empty command".to_owned()));
    }
    //check command is first token
    let command = parts[0].to_uppercase();
    match command.as_str() {
        "GET" => {
            if parts.len() != 2 {
                return Err(ParseError::WrongNumberOfArguments(format!(
                    "GET requires 2 arguments,incl. command; you had :{}",
                    parts.len()
                )));
            }
            return Ok(Command::Get {
                key: parts[1].to_string(),
            });
        }
        "SET" => {
            if parts.len() != 3 {
                return Err(ParseError::WrongNumberOfArguments(format!(
                    "SET requires 3 arguments, including command; you had: {}",
                    parts.len()
                )));
            }
            return Ok(Command::Set {
                key: parts[1].to_string(),
                val: parts[2].to_string(),
            });
        }
        "DEL" => {
            if parts.len() != 2 {
                return Err(ParseError::WrongNumberOfArguments(format!(
                    "DEL requires 2 arguments,incl. command; you had :{}",
                    parts.len()
                )));
            }
            return Ok(Command::Del {
                key: parts[1].to_string(),
            });
        }
        other => {
            return Err(ParseError::InvalidCommand(format!(
                "Uknown command: {other}"
            )));
        }
    }

    //get datastore
    //return value or error if not present
}
