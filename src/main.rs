#[allow(unused_imports)]
use std::io::{self, Write};

mod runnable;
mod utils;

fn main() {
    // TODO: Uncomment the code below to pass the first stage

    let commands = runnable::get_commands();
    
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut parts: Vec<String> = Vec::new();
        let mut buffer = String::new();
        let mut in_quotes = false;
        for part in input.trim().chars() {
            if part == '\'' {
                in_quotes = !in_quotes;
                continue;
            }
            if part.is_whitespace() && !in_quotes {
                if !buffer.is_empty() {
                    parts.push(buffer.clone());
                    buffer.clear();
                }
            } else {
                buffer.push(part);
            }
        }
        if !buffer.is_empty() {
            parts.push(buffer);
        }
        let command = &parts[0];
        let args: Vec<&str> = parts[1..]
            .iter()
            .map(|s| s.as_str())
            .collect();
        runnable::dispatch(&commands, command, args.as_slice());
    }
}
