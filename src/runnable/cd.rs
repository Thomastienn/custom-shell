use crate::runnable::{CommandContext, Runnable};
use std::{env, path::Path};

pub struct Cd;

impl Runnable for Cd {
    fn name(&self) -> &'static str {
        "cd"
    }

    fn run(&self, args: &[&str], _ctx: CommandContext) -> i32 {
        let path = Path::new(args[0]);
        if path.exists() && path.is_dir() {
            env::set_current_dir(path).unwrap();
            return 0;
        }
        println!("cd: {}: No such file or directory", args[0]);
        1
    }
}

