use crate::db::RustyDb;

mod command;
mod db;
mod err_types;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("RustyDB Sea Ally");
    println!("Type 'help' for commands, 'exit' to quit\n");

    let mut db = RustyDb::new(".rusty.db")?;
    loop {
        //main cli loop
    }
}
