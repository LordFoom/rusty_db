use std::io::Write;

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
        println!("rustydb>> ");
        std::io::stdout().flush()?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        //check for commands that aren't sql
        match input {}
    }
}
