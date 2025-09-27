use std::collections::HashMap;

use crate::err_types::RustyDbErr;
type Result<T> = std::result::Result<T, RustyDbErr>;

struct RustyDb {
    data: HashMap<String, String>,
}

impl RustyDb {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    fn get(&self, key: &str) -> Result<&String> {
        self.data
            .get(key)
            .ok_or_else(|| RustyDbErr::KeyNotFound(key.to_string()))
    }

    fn put(&mut self, key: String, val: String) -> Result<()> {
        self.data.insert(key.clone(), val.clone());
        Ok(())
        // .ok_or_else(|| RustyDbErr::SerializationError(key, val))
    }

    fn delete(&mut self, key: &str) -> Result<String> {
        self.data
            .remove(key)
            .ok_or_else(|| RustyDbErr::KeyNotFound(key.to_string()))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_put_and_get() -> Result<()> {
        let mut db = RustyDb::new();
        db.put("key1".to_string(), "val1".to_string())?;
        assert_eq!(Ok(&"val1".to_string()), db.get("key1"));
        Ok(())
    }
}
