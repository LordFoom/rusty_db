pub enum Command {
    Get { key: String },
    Set { key: String, val: String },
    Del { key: String },
}

///TODO String or &str?
pub fn parse(input: &str) -> Result<()> {
    //split into components (tokens)
    //check command is first token
    //get datastore
    //return value or error if not present
    Ok(())
}
