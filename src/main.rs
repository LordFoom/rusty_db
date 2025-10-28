use std::io::Write;

use crate::{command::parse, db::RustyDb};

mod command;
mod db;
mod err_types;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("RustyDB Sea Ally");
    println!("Type 'help' for commands, 'exit' to quit\n");

    let mut db = RustyDb::new(".rusty.db")?;
    loop {
        //main cli loop
        print!("rustydb>> ");
        std::io::stdout().flush()?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let input = input.trim();

        //check for commands that aren't sql
        match input.to_lowercase().as_str() {
            "exit" | "quit" | "q" => {
                println!("See you later, Space Cowboy");
                break;
            }
            "help" => {
                print_help();
                continue;
            }
            _ => match parse(&input) {
                Ok(cmd) => match db.execute(cmd) {
                    Ok(result) => print!("{}", result),
                    Err(why) => eprintln!("ERROR: {}", why),
                },
                Err(why) => eprintln!("Parser error: {}", why),
            },
        }
    }
    Ok(())
}

fn print_help() {
    println!("Available commands:");
    println!("  CREATE <table>             - Create a new table");
    println!("  DROP <table>               - Drop a table");
    println!("  LIST                       - List all tables");
    println!("  SET <table> <key> <value>  - Set a key-value pair");
    println!("  GET <table> <key>          - Get a value by key");
    println!("  DEL <table> <key>          - Delete a key");
    println!("  help                       - Show this help");
    println!("  exit                       - Exit the REPL");
}
