use serde::{Serialize, Deserialize}
use bincode::{Encode, Decode};


///Write Ahead Log entry
pub enum WalEntry {
    Put{ table:String, key:String, val:String, },
    Delete{ table: String, key: String},
    CreateTable{table: String},
    DropTable{table: String},
}
