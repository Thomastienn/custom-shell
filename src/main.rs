use crate::{input::InputCtx, utils::path::PathUtils};
use input::Input;
use parser::parse;
use runnable::CommandContext;
#[allow(unused_imports)]
use std::io::{self, ErrorKind, Write};
use structures::trie::Trie;
use tokenizer::Tokenizer;

mod input;
mod parser;
mod runnable;
mod structures;
mod tokenizer;
mod utils;

fn main() {
    let commands = runnable::get_commands();
    let mut cmd_trie = Trie::new();
    for cmd in commands.keys() {
        cmd_trie.insert(cmd);
    }
    let mut filesystem_trie = Trie::new();
    for entry in PathUtils::all_entries_rec_here() {
        // dbg!(PathUtils::get_relative_path(&entry.canonicalize().ok().unwrap()).unwrap());
        let is_dir = entry.is_dir();

        let full_path = entry.canonicalize().ok().unwrap();
        let mut rel = PathUtils::get_relative_path(&full_path).unwrap();

        if is_dir {
            rel.push('/');
        }

        filesystem_trie.insert(&rel);
    }

    loop {
        let input_ctx = InputCtx {
            cmd_pref: &cmd_trie,
            filesystem_pref: &filesystem_trie,
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
                    file_trie: &mut filesystem_trie,
                };
                runnable::dispatch(ctx);
            }
            Err(e) => eprintln!("Error parsing command: {}", e),
        }
    }
}
