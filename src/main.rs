use std::io::{self, Write};

// Declare the modules we created. Rust will look for `parser.rs` and `store.rs`.
mod parser;
mod store;

// Bring the necessary items into scope.
use parser::{parse_input, Command};
use store::KvStore;

fn main() {
    // The long-lived store is now an instance of our `KvStore` struct.
    let mut kv_store = KvStore::new();

    println!("Welcome to the Rusty Key-Value Store!");
    println!("Available commands: SET <key> <value>, GET <key>, DELETE <key>, EXIT");

    // The REPL loop
    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Delegate parsing to the parser module.
        let command = parse_input(&input);

        // The main logic is now a clean `match` on the structured `Command` enum.
        match command {
            Command::Set { key, value } => {
                kv_store.set(key, value);
                println!("OK");
            }
            Command::Get { key } => {
                // Note: We pass a slice `&key` to `get`.
                match kv_store.get(&key) {
                    Some(value) => println!("{}", value),
                    None => println!("(nil)"),
                }
            }
            Command::Delete { key } => {
                if kv_store.delete(&key).is_some() {
                    println!("Deleted");
                } else {
                    println!("Key not found");
                }
            }
            Command::Exit => {
                println!("Exiting. Goodbye!");
                break;
            }
            Command::Invalid(err_msg) => {
                if !err_msg.is_empty() {
                    println!("Error: {}", err_msg);
                }
            }
        }
    }
}
