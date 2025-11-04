use std::{collections::HashMap, fs, path::Path};

use bincode::{Decode, Encode, config, decode_from_slice, encode_to_vec};

use crate::{command::Command, err_types::RustyDbErr, wal::WalEntry};
type Result<T> = std::result::Result<T, RustyDbErr>;

#[derive(Debug, Encode, Decode)]
pub struct RustyDb {
    // data: HashMap<String, String>,
    pub tables: HashMap<String, HashMap<String, String>>,
    ///DB location on the filesyystem
    pub file_path: String,
    ///write ahead log path
    pub wal_path: String,
}

impl RustyDb {
    pub fn write_wal(&mut self, entry: &WalEntry) -> Result<()> {
        let config = config::standard();
        let encoded = encode_to_vec(entry, config)
            .map_err(|e| RustyDbErr::SerializationError(e.to_string()))?;
        Ok(())
    }

    pub fn new(file_path: &str) -> Result<Self> {
        let mut wal_path = format!("{}.wal", file_path);
        let mut rusty_db = Self {
            // data: HashMap::new(),
            tables: HashMap::new(),
            file_path: file_path.to_string(),
            wal_path: wal_path.clone(),
        };

        if Path::new(file_path).exists() {
            rusty_db.load_from_disk()?;
        }

        if Path::new(&wal_path).exists() {}
        Ok(rusty_db)
    }

    pub fn execute(&mut self, cmd: Command) -> Result<String> {
        match cmd {
            Command::Get { table, key } => {
                let val = self.get(&table, &key)?;
                Ok(format!("{}", val))
            }
            Command::Put { table, key, val } => {
                self.put(table, key, val)?;
                Ok("Ok".to_string())
            }
            Command::Del { table, key } => {
                let val = self.delete(&table, &key)?;
                Ok(format!("{}", val))
            }
            Command::CreateTable { table_name } => {
                self.create_table(&table_name)?;
                Ok(format!("Created table {}", table_name))
            }
            Command::DropTable { table_name } => {
                self.drop_table(&table_name)?;
                Ok(format!("Dropped table {}", table_name))
            }
            Command::ListTables => {
                let list_tables = self.list_tables();
                if list_tables.is_empty() {
                    return Ok("No tables found".to_string());
                }
                Ok(list_tables.join("\n"))
            }
        }
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
            .get_mut(&table)
            .ok_or_else(|| RustyDbErr::TableNotFound(table))?
            .insert(key, val);
        self.save_to_disk()?;
        Ok(())
    }

    ///Delete a value from a table
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

    ///Create a table
    pub fn create_table(&mut self, table_name: &str) -> Result<()> {
        if self.tables.contains_key(table_name) {
            return Err(RustyDbErr::TableExists(table_name.to_string()));
        }
        self.tables.insert(table_name.to_string(), HashMap::new());
        self.save_to_disk()?;
        Ok(())
    }

    ///Drop a table
    pub fn drop_table(&mut self, table_name: &str) -> Result<()> {
        if self.tables.contains_key(table_name) {
            return Err(RustyDbErr::TableNotFound(table_name.to_string()));
        }
        self.tables.remove(table_name);
        self.save_to_disk()?;
        Ok(())
    }

    ///List all the tables
    pub fn list_tables(&self) -> Vec<String> {
        self.tables.keys().map(|key| key.to_string()).collect()
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

    ///Replay the wal to reconstruct data
    pub fn replay_wal(&mut self) -> Result<()> {
        let data = fs::read(&self.wal_path).map_err(|e| RustyDbErr::IoError(e.to_string()))?;

        let mut offset = 0;
        let config = config::standard();

        while offset < data.len() {
            //read length prefix
            if offset + 4 > data.len() {
                break; //incomplete entry at end of file?
            }

            let len = u32::from_le_bytes([
                data[offset],
                data[offset + 1],
                data[offset + 2],
                data[offset + 3],
            ]) as usize;
            offset += 4;
            //check if we have the full entry
            if offset + len > data.len() {
                //incomplete final entry
                break;
            }

            //decode the entry
            let entry_data = &data[offset..offset + len];
            let (entry, __): (WalEntry, usize) = decode_from_slice(entry_data, config)
                .map_err(|e| RustyDbErr::SerializationError(e.to_string()))?;
            //apply the entry to in-memory state
            self.apply_wal_entry(&entry)?;
        }
        Ok(())
    }

    pub fn apply_wal_entry(&mut self, entry: &WalEntry) -> Result<()> {
        match entry {
            WalEntry::Put { table, key, val } => {
                self.tables
                    .entry(table.to_string())
                    //we are lenient during replay_wal
                    .or_insert_with(HashMap::new)
                    .insert(key.to_string(), val.to_string());
            }
            WalEntry::Delete { table, key } => {
                if let Some(t) = self.tables.get_mut(table) {
                    t.remove(key);
                }
            }
            WalEntry::CreateTable { table } => {
                self.tables
                    .entry(table.to_string())
                    .or_insert_with(HashMap::new);
            }
            WalEntry::DropTable { table } => {
                self.tables.remove(table);
            }
        }
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
    fn test_put_to_nonexistent_table() {
        let path = test_db_path("put_no_table");
        cleanup(&path);
        let mut db = RustyDb::new(&path).unwrap();
        let result = db.put(
            "missing_table".to_string(),
            "key1".to_string(),
            "val1".to_string(),
        );
        assert_eq!(
            result,
            Err(RustyDbErr::TableNotFound("missing_table".to_string()))
        );
        cleanup(&path);
    }

    #[test]
    fn test_create_table_then_put() -> Result<()> {
        let path = test_db_path("create_then_put");
        cleanup(&path);
        let mut db = RustyDb::new(&path)?;
        db.create_table("test_table")?;
        db.put(
            "test_table".to_string(),
            "key1".to_string(),
            "val1".to_string(),
        )?;
        assert_eq!(db.get("test_table", "key1")?, &"val1".to_string());
        cleanup(&path);
        Ok(())
    }

    #[test]
    fn test_get_non_existent_key() {
        let path = test_db_path("nonexistent");
        cleanup(&path);
        let mut db = RustyDb::new(&path).unwrap();
        db.create_table("test_table").unwrap();
        db.put(
            "test_table".to_string(),
            "existing".to_string(),
            "this_is_my_key".to_string(),
        )
        .unwrap();
        let unval = db.get("test_table", "missing");
        assert_eq!(unval, Err(RustyDbErr::KeyNotFound("missing".to_string())));
        cleanup(&path);
    }

    #[test]
    fn test_put_and_get() -> Result<()> {
        let path = test_db_path("basic");
        cleanup(&path);
        let mut db = RustyDb::new(&path)?;
        db.create_table("test_table")?;
        db.put(
            "test_table".to_string(),
            "key1".to_string(),
            "val1".to_string(),
        )?;
        assert_eq!(Ok(&"val1".to_string()), db.get("test_table", "key1"));
        cleanup(&path);
        Ok(())
    }

    #[test]
    fn test_get_from_nonexistent_table() {
        let path = test_db_path("no_table");
        cleanup(&path);
        let db = RustyDb::new(&path).unwrap();
        let result = db.get("missing_table", "key");
        assert_eq!(
            result,
            Err(RustyDbErr::TableNotFound("missing_table".to_string()))
        );
        cleanup(&path);
    }

    #[test]
    fn test_delete_key() -> Result<()> {
        let path = test_db_path("delete");
        cleanup(&path);
        let mut db = RustyDb::new(&path)?;
        db.create_table("test_table")?;
        db.put(
            "test_table".to_string(),
            "key1".to_string(),
            "val1".to_string(),
        )?;
        let deleted = db.delete("test_table", "key1")?;
        assert_eq!(deleted, "val1");
        assert_eq!(
            db.get("test_table", "key1"),
            Err(RustyDbErr::KeyNotFound("key1".to_string()))
        );
        cleanup(&path);
        Ok(())
    }

    #[test]
    fn test_delete_nonexistent_key() {
        let path = test_db_path("delete_missing");
        cleanup(&path);
        let mut db = RustyDb::new(&path).unwrap();
        db.create_table("test_table").unwrap();
        db.put(
            "test_table".to_string(),
            "key1".to_string(),
            "val1".to_string(),
        )
        .unwrap();
        let result = db.delete("test_table", "missing_key");
        assert_eq!(
            result,
            Err(RustyDbErr::KeyNotFound("missing_key".to_string()))
        );
        cleanup(&path);
    }

    #[test]
    fn test_delete_from_nonexistent_table() {
        let path = test_db_path("delete_no_table");
        cleanup(&path);
        let mut db = RustyDb::new(&path).unwrap();
        let result = db.delete("missing_table", "key");
        assert_eq!(
            result,
            Err(RustyDbErr::TableNotFound("missing_table".to_string()))
        );
        cleanup(&path);
    }

    #[test]
    fn test_create_table() -> Result<()> {
        let path = test_db_path("create_table");
        cleanup(&path);
        let mut db = RustyDb::new(&path)?;
        db.create_table("new_table")?;
        let tables = db.list_tables();
        assert!(tables.contains(&"new_table".to_string()));
        cleanup(&path);
        Ok(())
    }

    #[test]
    fn test_create_existing_table() {
        let path = test_db_path("create_existing");
        cleanup(&path);
        let mut db = RustyDb::new(&path).unwrap();
        db.create_table("test_table").unwrap();
        let result = db.create_table("test_table");
        assert_eq!(
            result,
            Err(RustyDbErr::TableExists("test_table".to_string()))
        );
        cleanup(&path);
    }

    #[test]
    fn test_list_tables() -> Result<()> {
        let path = test_db_path("list_tables");
        cleanup(&path);
        let mut db = RustyDb::new(&path)?;
        db.create_table("table1")?;
        db.create_table("table2")?;
        db.create_table("table3")?;
        let tables = db.list_tables();
        assert_eq!(tables.len(), 3);
        assert!(tables.contains(&"table1".to_string()));
        assert!(tables.contains(&"table2".to_string()));
        assert!(tables.contains(&"table3".to_string()));
        cleanup(&path);
        Ok(())
    }

    #[test]
    fn test_persistence() -> Result<()> {
        let path = test_db_path("persistence");
        cleanup(&path);
        {
            let mut db = RustyDb::new(&path)?;
            db.create_table("users")?;
            db.put(
                "users".to_string(),
                "user1".to_string(),
                "alice".to_string(),
            )?;
            db.put("users".to_string(), "user2".to_string(), "bob".to_string())?;
        }
        {
            let db = RustyDb::new(&path)?;
            assert_eq!(db.get("users", "user1")?, &"alice".to_string());
            assert_eq!(db.get("users", "user2")?, &"bob".to_string());
        }
        cleanup(&path);
        Ok(())
    }

    #[test]
    fn test_multiple_tables() -> Result<()> {
        let path = test_db_path("multi_tables");
        cleanup(&path);
        let mut db = RustyDb::new(&path)?;
        db.create_table("users")?;
        db.create_table("posts")?;
        db.put("users".to_string(), "id1".to_string(), "alice".to_string())?;
        db.put(
            "posts".to_string(),
            "id1".to_string(),
            "hello world".to_string(),
        )?;
        assert_eq!(db.get("users", "id1")?, &"alice".to_string());
        assert_eq!(db.get("posts", "id1")?, &"hello world".to_string());
        cleanup(&path);
        Ok(())
    }

    #[test]
    fn test_update_value() -> Result<()> {
        let path = test_db_path("update");
        cleanup(&path);
        let mut db = RustyDb::new(&path)?;
        db.create_table("test_table")?;
        db.put(
            "test_table".to_string(),
            "key1".to_string(),
            "val1".to_string(),
        )?;
        db.put(
            "test_table".to_string(),
            "key1".to_string(),
            "val2".to_string(),
        )?;
        assert_eq!(db.get("test_table", "key1")?, &"val2".to_string());
        cleanup(&path);
        Ok(())
    }

    #[test]
    fn test_empty_list_tables() -> Result<()> {
        let path = test_db_path("empty_list");
        cleanup(&path);
        let db = RustyDb::new(&path)?;
        let tables = db.list_tables();
        assert_eq!(tables.len(), 0);
        cleanup(&path);
        Ok(())
    }

    #[test]
    fn test_multiple_puts_same_table() -> Result<()> {
        let path = test_db_path("multi_puts");
        cleanup(&path);
        let mut db = RustyDb::new(&path)?;
        db.create_table("test_table")?;
        db.put(
            "test_table".to_string(),
            "key1".to_string(),
            "val1".to_string(),
        )?;
        db.put(
            "test_table".to_string(),
            "key2".to_string(),
            "val2".to_string(),
        )?;
        db.put(
            "test_table".to_string(),
            "key3".to_string(),
            "val3".to_string(),
        )?;
        assert_eq!(db.get("test_table", "key1")?, &"val1".to_string());
        assert_eq!(db.get("test_table", "key2")?, &"val2".to_string());
        assert_eq!(db.get("test_table", "key3")?, &"val3".to_string());
        cleanup(&path);
        Ok(())
    }
}
