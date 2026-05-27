use crate::{input::InputCtx, runnable::{cd::Cd, jobs::Jobs}, structures::dll::DoublyLinkedList,};
use input::InputShell;
use runnable::ShellContext;
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
    let mut job_list = DoublyLinkedList::new();
    let mut history = Vec::new();

    loop {
        let input_ctx = InputCtx {
            _commands: &commands,
            completions_path: &completions_path,
            cmd_pref: &cmd_trie,
            filesystem_pref: &filesystem_trie,
            history: &mut history,
        };
        let removed_jobs = Jobs::reap_jobs(&mut job_list);
        for job_str in removed_jobs {
            println!("{}", job_str);
        }
        let input_res = InputShell::read_line("$ ", input_ctx);
        match input_res {
            Ok(parsed_command) => {
                // dbg!("Parsed command: {:?}", &parsed_command);
                let ctx = ShellContext {
                    commands_map: &commands,
                    completions_path: &mut completions_path,
                    file_trie: &mut filesystem_trie,
                    job_list: &mut job_list,
                    history: &history,
                };
                runnable::dispatch(ctx, parsed_command);
            }
            Err(e) if e.kind() == ErrorKind::Interrupted => {
                println!();
                break;
            }
            Err(e) => eprintln!("Error parsing command: {}", e),
        }
    }
}
