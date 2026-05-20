use crate::runnable::{CommandContext, Runnable};
use crate::utils::output;
use std::env;

pub struct Pwd;

impl Runnable for Pwd {
    fn name(&self) -> &'static str {
        "pwd"
    }

    fn run(&self, _args: &[&str], _ctx: &CommandContext) -> i32 {
        let content = env::current_dir().unwrap().display().to_string();
        let output = &_ctx.stdout;
        match output::write_to_output(output, content.as_str()) {
            Ok(_) => return 0,
            Err(e) => {
                eprintln!("Error writing to output: {}", e);
                return 1;
            }
        }
    }
}

