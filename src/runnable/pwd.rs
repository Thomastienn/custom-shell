use crate::runnable::{CommandContext, Runnable};
use std::env;

pub struct Pwd;

impl Runnable for Pwd {
    fn name(&self) -> &'static str {
        "pwd"
    }

    fn run(&self, _args: &[&str], _ctx: CommandContext) -> i32 {
        println!("{}", env::current_dir().unwrap().display());
        0
    }
}

