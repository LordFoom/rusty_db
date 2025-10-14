use std::{collections::HashMap, fs, path::Path};

use bincode::{Decode, Encode, config, decode_from_reader, encode_to_vec};

use crate::err_types::RustyDbErr;
type Result<T> = std::result::Result<T, RustyDbErr>;

#[derive(Debug, Encode, Decode)]
struct RustyDb {
    // data: HashMap<String, String>,
    tables: HashMap<String, HashMap<String, String>>,
    ///DB location on the filesyystem
    file_path: String,
}

impl RustyDb {
    pub fn new(file_path: &str) -> Result<Self> {
        let mut rusty_db = Self {
            // data: HashMap::new(),
            tables: HashMap::new(),
            file_path: file_path.to_string(),
        };
        if Path::new(file_path).exists() {
            rusty_db.load_from_disk()?;
        }
        Ok(rusty_db)
    }

    pub fn get(&self, table: &str, key: &str) -> Result<&String> {
        self.tables
            .get(table)
            .ok_or_else(|| RustyDbErr::TableNotFound(table.to_string()))?
            .get(key)
            .ok_or_else(|| RustyDbErr::KeyNotFound(key.to_string()))
    }

    pub fn put(&mut self, table: String, key: String, val: String) -> Result<()> {
        self.tables
            .entry(table)
            .or_insert(HashMap::new())
            // .ok_or_else(|| RustyDbErr::TableNotFound(table.to_string()))?
            .insert(key, val);
        self.save_to_disk()?;
        Ok(())
        // .ok_or_else(|| RustyDbErr::SerializationError(key, val))
    }

    pub fn delete(&mut self, table: &str, key: &str) -> Result<String> {
        let deleted = self
            .tables
            .get_mut(table)
            .ok_or_else(|| RustyDbErr::TableNotFound(table.to_string()))?
            .remove(key)
            .ok_or_else(|| RustyDbErr::KeyNotFound(key.to_string()))?;
        self.save_to_disk()?;
        Ok(deleted)
    }

    pub fn save_to_disk(&mut self) -> Result<()> {
        let config = config::standard();
        let encoded = encode_to_vec(&self.tables, config)
            .map_err(|e| RustyDbErr::SerializationError(e.to_string()))?;

        fs::write(&self.file_path, encoded).map_err(|e| RustyDbErr::IoError(e.to_string()))?;

        Ok(())
    }

    pub fn load_from_disk(&mut self) -> Result<()> {
        let config = config::standard();
        let data = fs::read(&self.file_path).map_err(|e| RustyDbErr::IoError(e.to_string()))?;
        let (decoded, _len): (HashMap<String, HashMap<String, String>>, usize) =
            bincode::decode_from_slice(&data, config)
                .map_err(|e| RustyDbErr::SerializationError(e.to_string()))?;

        self.tables = decoded;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_db_path(name: &str) -> String {
        format!("/tmp/rusty_db_{}.bin", name)
    }

    fn cleanup(path: &str) {
        fs::remove_file(path).ok();
    }

    #[test]
    fn test_get_non_existent_key() {
        let path = test_db_path("nonexistent");
        cleanup(&path);
        let mut db = RustyDb::new(&path).unwrap();
        db.put(
            "test_table".to_string(),
            "existing".to_string(),
            "this_is_my_key".to_string(),
        )
        .unwrap();
        let unval = db.get("test_table", "missing");
        assert_eq!(unval, Err(RustyDbErr::KeyNotFound("missing".to_string())));
    }

    #[test]
    fn test_put_and_get() -> Result<()> {
        let path = test_db_path("basic");
        cleanup(&path);
        let mut db = RustyDb::new(&path)?;
        db.put(
            "test_table".to_string(),
            "key1".to_string(),
            "val1".to_string(),
        )?;
        assert_eq!(Ok(&"val1".to_string()), db.get("test_table", "key1"));
        Ok(())
    }
}
