use crate::runnable::{CommandContext, Runnable};
use std::{env, path::PathBuf};

pub struct Cd;

impl Runnable for Cd {
    fn name(&self) -> &'static str {
        "cd"
    }

    fn run(&self, args: &[&str], _ctx: &CommandContext) -> i32 {
        let mut path = PathBuf::from(args[0]);
        if path.starts_with("~") {
            if let Ok(home) = env::var("HOME") {
                path = PathBuf::from(home).join(path.strip_prefix("~").unwrap());
            } else {
                return 1;
            }
        }
        if path.exists() && path.is_dir() {
            env::set_current_dir(path).unwrap();
            return 0;
        }
        eprintln!("cd: {}: No such file or directory", args[0]);
        1
    }
}

