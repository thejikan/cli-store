/// Represents the structured commands our application can execute.
/// Using an enum makes our main logic much cleaner and safer, as the
/// compiler will ensure we handle all possible command variants.
#[derive(Debug)]
pub enum Command {
    Set { key: String, value: String },
    Get { key: String },
    Delete { key: String },
    Exit,
    Invalid(String),
}

/// Parses a raw input string slice into a `Command`.
pub fn parse_input(input: &str) -> Command {
    let mut parts = input.trim().split_whitespace();
    let command_name = parts.next();

    match command_name {
        Some("SET") => {
            // The ownership handoff from `&str` to `String` happens here,
            // inside the parser. This is a logical place for it.
            let key = parts.next().map(|s| s.to_string());
            // Collect the rest of the parts to handle values with spaces
            let value = parts.collect::<Vec<&str>>().join(" ");

            if let (Some(k), false) = (key, value.is_empty()) {
                Command::Set { key: k, value }
            } else {
                Command::Invalid("SET requires a key and a value.".to_string())
            }
        }
        Some("GET") => {
            if let Some(key) = parts.next() {
                Command::Get {
                    key: key.to_string(),
                }
            } else {
                Command::Invalid("GET requires a key.".to_string())
            }
        }
        Some("DELETE") => {
            if let Some(key) = parts.next() {
                Command::Delete {
                    key: key.to_string(),
                }
            } else {
                Command::Invalid("DELETE requires a key.".to_string())
            }
        }
        Some("EXIT") | Some("QUIT") => Command::Exit,
        // Handle empty input
        None => Command::Invalid("".to_string()), // Or a different variant if you prefer
        // Handle any other command
        Some(cmd) => Command::Invalid(format!("Unknown command '{}'", cmd)),
    }
}