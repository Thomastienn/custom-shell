#[allow(unused_imports)]
use std::io::{self, Write};
use runnable::CommandContext;
use tokenizer::Tokenizer;
use parser::parse;

mod runnable;
mod utils;
mod tokenizer;
mod parser;

fn main() {
    let commands = runnable::get_commands();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim().is_empty() {
            continue;
        }
        
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize();

        match parse(tokens) {
            Ok(parsed_command) => {
                // dbg!("Parsed command: {:?}", &parsed_command);
                let ctx = CommandContext {
                    commands: &commands,
                    parsed_command: &parsed_command,
                };
                runnable::dispatch(ctx);
            },
            Err(e) => eprintln!("Error parsing command: {}", e),
        }
    }
}
