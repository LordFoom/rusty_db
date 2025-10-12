use crate::err_types::ParseError;

pub enum Command {
    Get {
        table: String,
        key: String,
    },
    Set {
        table: String,
        key: String,
        val: String,
    },
    Del {
        table: String,
        key: String,
    },
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
            if parts.len() != 3 {
                return Err(ParseError::WrongNumberOfArguments(format!(
                    "GET requires 2 arguments, table and key, you had:{}",
                    parts.len()
                )));
            }
            return Ok(Command::Get {
                table: parts[1].to_string(),
                key: parts[2].to_string(),
            });
        }
        "SET" => {
            if parts.len() != 3 {
                return Err(ParseError::WrongNumberOfArguments(format!(
                    "SET requires 3 arguments,table, key val; you had: {}",
                    parts.len()
                )));
            }
            return Ok(Command::Set {
                table: parts[1].to_string(),
                key: parts[2].to_string(),
                val: parts[3].to_string(),
            });
        }
        "DEL" => {
            if parts.len() != 2 {
                return Err(ParseError::WrongNumberOfArguments(format!(
                    "DEL requires 2 arguments,table, key, you had: {}",
                    parts.len()
                )));
            }
            return Ok(Command::Del {
                table: parts[1].to_string(),
                key: parts[2].to_string(),
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

#[cfg(test)]
mod tests {
    use crate::err_types::RustyDbErr;

    use super::*;

    #[test]
    fn test_parse_get() {
        let input = "get table1 key1";
        let result = parse(input);
        assert!(result.is_ok());

        if let Ok(Command::Get { table, key }) = result {
            assert_eq!("table1", table);
            assert_eq!("key1", key);
        } else {
            panic!("Expected Command::Get from {}", input);
        }
    }
}
