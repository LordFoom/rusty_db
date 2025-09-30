use std::{collections::HashMap, fs};

use bincode::{Decode, Encode, config, decode_from_reader, encode_to_vec};

use crate::err_types::RustyDbErr;
type Result<T> = std::result::Result<T, RustyDbErr>;

#[derive(Debug, Encode, Decode)]
struct RustyDb {
    data: HashMap<String, String>,
    ///DB location on the filesyystem
    file_path: String,
}

impl RustyDb {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            file_path: String::new(),
        }
    }

    pub fn get(&self, key: &str) -> Result<&String> {
        self.data
            .get(key)
            .ok_or_else(|| RustyDbErr::KeyNotFound(key.to_string()))
    }

    pub fn put(&mut self, key: String, val: String) -> Result<()> {
        self.data.insert(key.clone(), val.clone());
        Ok(())
        // .ok_or_else(|| RustyDbErr::SerializationError(key, val))
    }

    pub fn delete(&mut self, key: &str) -> Result<String> {
        self.data
            .remove(key)
            .ok_or_else(|| RustyDbErr::KeyNotFound(key.to_string()))
    }

    pub fn save_to_disk(&mut self) -> Result<()> {
        let config = config::standard();
        let encoded = encode_to_vec(&self.data, config)
            .map_err(|e| RustyDbErr::SerializationError(e.to_string()))?;

        fs::write(&self.file_path, encoded).map_err(|e| RustyDbErr::IoError(e.to_string()))?;

        Ok(())
    }

    pub fn load_from_disk(&mut self) -> Result<()> {
        let config = config::standard();
        let data = fs::read(&self.file_path).map_err(|e| RustyDbErr::IoError(e.to_string()))?;
        let (decoded, _len): (HashMap<String, String>, usize) =
            bincode::decode_from_slice(&data, config)
                .map_err(|e| RustyDbErr::SerializationError(e.to_string()))?;

        self.data = decoded;
        Ok(())
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
