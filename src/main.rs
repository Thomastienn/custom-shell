#[allow(unused_imports)]
use std::io::{self, Write, ErrorKind};
use crate::{input::InputCtx, utils::path::PathUtils};
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
    let mut cmd_trie = Trie::new();
    for cmd in commands.keys() {
        cmd_trie.insert(cmd);
    }
    let mut file_trie = Trie::new();
    for file in PathUtils::all_files() {
        file_trie.insert(PathUtils::get_filename(&file).unwrap().as_str());
    }

    loop {
        let input_ctx = InputCtx {
            cmd_pref: &cmd_trie,
            file_pref: &file_trie,
        };
        let input_str = match Input::read_line("$ ", input_ctx) {
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
                    file_trie: &mut file_trie,
                };
                runnable::dispatch(ctx);
            },
            Err(e) => eprintln!("Error parsing command: {}", e),
        }
    }
}
