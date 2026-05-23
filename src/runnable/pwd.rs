use crate::runnable::{CommandContext, Runnable};
use crate::utils::output;
use std::env;

pub struct Pwd;

impl Runnable for Pwd {
    fn name(&self) -> String {
        "pwd".to_string()
    }

    fn run(&self, _args: &Vec<String>, ctx: CommandContext) -> i32 {
        let content = env::current_dir().unwrap().display().to_string();
        let stdout = &ctx.parsed_command.stdout;
        // let stderr = &ctx.parsed_command.stderr;

        return output::write(content.as_str(), stdout);
    }
}

