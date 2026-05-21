use crate::runnable::{CommandContext, Runnable};
use crate::utils::output;
use std::env;

pub struct Pwd;

impl Runnable for Pwd {
    fn name(&self) -> &'static str {
        "pwd"
    }

    fn run(&self, _args: &Vec<String>, ctx: CommandContext) -> i32 {
        let content = env::current_dir().unwrap().display().to_string();
        let stdout = &ctx.parsed_command.stdout;
        // let stderr = &ctx.parsed_command.stderr;
        
        match output::write_to_output(stdout, content.as_str()) {
            Ok(_) => return 0,
            Err(e) => {
                eprintln!("Error writing to error output: {}", e);
                return 1;
            }
        }
    }
}

