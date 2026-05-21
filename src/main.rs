#[allow(unused_imports)]
use std::io::{self, Write, ErrorKind};
use runnable::CommandContext;
use structures::trie::Trie;
use tokenizer::Tokenizer;
use parser::parse;
use input::Input;

mod runnable;
mod utils;
mod tokenizer;
mod parser;
mod structures;
mod input;

fn main() {
    let commands = runnable::get_commands();
    let mut trie = Trie::new();
    for cmd in commands.keys() {
        trie.insert(cmd);
    }

    let input_handler = Input::new(&trie);

    loop {
        let input_str = match input_handler.read_line("$ ") {
            Ok(line) => line,

            Err(e) => {
                eprintln!("input error: {e}");
                break;
            }
        };

        if input_str.trim().is_empty() {
            continue;
        }
        
        let mut tokenizer = Tokenizer::new(input_str);
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
