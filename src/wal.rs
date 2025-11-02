use std::fs;

use bincode::{Decode, Encode};

use crate::wal;

///Write Ahead Log entry
#[derive(Debug, Clone, Encode, Decode)]
pub enum WalEntry {
    Put {
        table: String,
        key: String,
        val: String,
    },
    Delete {
        table: String,
        key: String,
    },
    CreateTable {
        table: String,
    },
    DropTable {
        table: String,
    },
}

impl WalEntry {
    pub fn table_name(&self) -> &str {
        match self {
            WalEntry::Put { table, .. } => table,
            WalEntry::Delete { table, .. } => table,
            WalEntry::CreateTable { table } => table,
            WalEntry::DropTable { table } => table,
        }
    }
}
