use std::collections::HashMap;

struct RustyDb {
    data: HashMap<String, String>,
}

impl RustyDb {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}
fn main() {}
