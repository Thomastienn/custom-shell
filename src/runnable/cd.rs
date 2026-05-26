use crate::runnable::{ExecContext, RunResult, Runnable};
use crate::structures::trie::Trie;
use crate::utils::io;
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

    fn run(&self, ctx: ExecContext) -> RunResult {
        let args = &ctx.own_parsed_command.args;
        let mut path = PathBuf::from(args[0].as_str());
        let stderr = &ctx.own_parsed_command.stderr;
        let file_trie = &mut *ctx.shell_ctx.file_trie;
        
        if path.starts_with("~") {
            if let Ok(home) = env::var("HOME") {
                path = PathBuf::from(home).join(path.strip_prefix("~").unwrap());
            } else {
                return RunResult::exit(1);
            }
        }
        if path.exists() && path.is_dir() {
            env::set_current_dir(path).unwrap();

            file_trie.clear();
            Cd::build_filesystem_trie(file_trie);

            return RunResult::exit(0);
        }
        let content_error = format!("cd: {}: No such file or directory", args[0]);
        return io::error(content_error.as_str(), stderr, 1);
    }
}

