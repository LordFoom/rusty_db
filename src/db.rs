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

#[cfg(test)]
mod test {
    use super::RustyDb;

    #[test]
    fn test_put_and_get() {
        let mut db = RustyDb::new();
        db.put("key1".to_string(), "val1".to_string());
        assert_eq!(Some(&"val1".to_string()), db.get("key1"));
    }
}
