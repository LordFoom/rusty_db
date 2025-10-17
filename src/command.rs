use crate::err_types::{ParseError, RustyDbErr};

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
    CreateTable {
        table_name: String,
    },
    DropTable {
        table_name: String,
    },
    ListTables,
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
            check_len(&parts, 3, "GET requires 2 arguments, table and key")?;
            return Ok(Command::Get {
                table: parts[1].to_string(),
                key: parts[2].to_string(),
            });
        }
        "SET" => {
            check_len(&parts, 3, "SET requires 3 arguments,table, key, val")?;
            return Ok(Command::Set {
                table: parts[1].to_string(),
                key: parts[2].to_string(),
                val: parts[3].to_string(),
            });
        }
        "DEL" => {
            check_len(&parts, 3, "DEL requires 2 arguments,table, key")?;
            return Ok(Command::Del {
                table: parts[1].to_string(),
                key: parts[2].to_string(),
            });
        }
        "CREATE" => {
            check_len(&parts, 2, "CREATE requires 1 arguments,table_name")?;
            return Ok(Command::CreateTable {
                table_name: parts[1].to_string(),
            });
        }
        "DROP" => {
            check_len(&parts, 1, "DROP requires 1 arguments,table_name")?;
            return Ok(Command::DropTable {
                table_name: parts[1].to_string(),
            });
        }
        "LIST" => {}
        other => {
            return Err(ParseError::InvalidCommand(format!(
                "Uknown command: {other}"
            )));
        }
    }

    //get datastore
    //return value or error if not present
}

fn check_len(parts: &[&str], expected_num: usize, err_msg: &str) -> Result<(), ParseError> {
    if parts.len() != expected_num {
        return Err(ParseError::WrongNumberOfArguments(format!(
            "{}! actual-> {}",
            err_msg,
            parts.len()
        )));
    }
    Ok(())
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
