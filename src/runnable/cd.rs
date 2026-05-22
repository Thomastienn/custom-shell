use crate::runnable::{CommandContext, Runnable};
use crate::utils::output;
use crate::utils::path::PathUtils;
use std::{env, path::PathBuf};

pub struct Cd;

impl Runnable for Cd {
    fn name(&self) -> String {
        "cd".to_string()
    }

    fn run(&self, args: &Vec<String>, ctx: CommandContext) -> i32 {
        let mut path = PathBuf::from(args[0].as_str());
        let stderr = &ctx.parsed_command.stderr;
        let file_trie = ctx.file_trie;
        
        if path.starts_with("~") {
            if let Ok(home) = env::var("HOME") {
                path = PathBuf::from(home).join(path.strip_prefix("~").unwrap());
            } else {
                return 1;
            }
        }
        if path.exists() && path.is_dir() {
            env::set_current_dir(path).unwrap();

            file_trie.clear();
            for file in PathUtils::all_files() {
                file_trie.insert(PathUtils::get_filename(&file).unwrap().as_str());
            }

            return 0;
        }
        let content_error = format!("cd: {}: No such file or directory", args[0]);
        match output::write_to_output(stderr, content_error.as_str()) {
            Ok(_) => return 1,
            Err(e) => {
                eprintln!("Error writing to error output: {}", e);
                return 1;
            }
        }
    }
}

