use crate::err_types::ParseError;

pub enum Command {
    Get { key: String },
    Set { key: String, val: String },
    Del { key: String },
}

///TODO String or &str?
pub fn parse(input: &str) -> Result<()> {
    //split into components (tokens)
    let parts = input.trim().split_whitespace().collect::<Vec<&str>>();
    if parts.is_empty() {
        return Err(ParseError::InvalidCommand("Empty command".to_owned()));
    }
    //check command is first token
    let command = parts[0].to_uppercase();
    match command.as_str() {
        "GET" => println!("GET key"),
        "SET" => println!("SET key, val"),
        "DEL" => println!("DEL key"),
        other => {
            return Err(ParseError::InvalidCommand(format!(
                "Uknown command: {other}"
            )));
        }
    }

    //get datastore
    //return value or error if not present
    Ok(())
}
