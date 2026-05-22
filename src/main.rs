use crate::{input::InputCtx, runnable::cd::Cd };
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
    Cd::build_filesystem_trie(&mut filesystem_trie);

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
