///Not going to go this way
pub enum Query {
    SELECT { key: Option<String> },
    INSERT { key: String, val: String },
    UPDATE { key: String, val: String },
    DELETE { key: String },
}
