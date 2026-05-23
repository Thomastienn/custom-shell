use crate::runnable::{CommandContext, Runnable};
use crate::structures::trie::Trie;
use crate::utils::output;
use crate::utils::path::PathUtils;
use std::{env, path::PathBuf};

pub struct Cd;

impl Cd {
    pub fn build_filesystem_trie(trie: &mut Trie) {
        for entry in PathUtils::all_entries_rec_here() {
            let is_dir = entry.is_dir();

            let full_path = entry.canonicalize().ok().unwrap();
            let mut rel = PathUtils::get_relative_path(&full_path).unwrap();

            if is_dir {
                rel.push('/');
            }
            // dbg!(&rel);

            trie.insert(&rel);
        }
    }
}

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
            Cd::build_filesystem_trie(file_trie);

            return 0;
        }
        let content_error = format!("cd: {}: No such file or directory", args[0]);
        return output::error(content_error.as_str(), stderr, 1);
    }
}

