pub enum Command {
    Get { key: String },
    Set { key: String, val: String },
    Del { key: String },
}

pub fn parse() {}
