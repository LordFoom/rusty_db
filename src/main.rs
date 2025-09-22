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

    fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    fn put(&mut self, key: String, val: String) {
        self.data.insert(key, val);
    }

    fn delete(&mut self, key: &str) {
        self.data.remove(key);
    }
}
fn main() {}
