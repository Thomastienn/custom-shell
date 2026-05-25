use crate::{input::InputCtx, runnable::{cd::Cd, jobs::Jobs}, structures::dll::DoublyLinkedList,};
use input::Input;
use runnable::CommandContext;
use std::collections::HashMap;
#[allow(unused_imports)]
use std::io::{self, ErrorKind, Write};
use structures::trie::Trie;

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

    let mut completions_path = HashMap::new();
    let mut cnt_bg = 0;
    let mut job_list = DoublyLinkedList::new();

    loop {
        let input_ctx = InputCtx {
            _commands: &commands,
            completions_path: &completions_path,
            cmd_pref: &cmd_trie,
            filesystem_pref: &filesystem_trie,
        };
        let removed_jobs = Jobs::reap_jobs(&mut job_list);
        for job_str in removed_jobs {
            println!("{}", job_str);
        }
        let input_res = Input::read_line("$ ", input_ctx);
        match input_res {
            Ok(parsed_command) => {
                // dbg!("Parsed command: {:?}", &parsed_command);
                cnt_bg += if parsed_command.background { 1 } else { 0 };
                let ctx = CommandContext {
                    commands: &commands,
                    completions_path: &mut completions_path,
                    parsed_command: &parsed_command,
                    file_trie: &mut filesystem_trie,
                    cnt_bg: cnt_bg,
                    job_list: &mut job_list,
                };
                runnable::dispatch(ctx);
            }
            Err(e) if e.kind() == ErrorKind::Interrupted => {
                println!();
                break;
            }
            Err(e) => eprintln!("Error parsing command: {}", e),
        }
    }
}
