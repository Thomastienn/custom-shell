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
        let stdout = &_ctx.stdout;
        let stderr = &_ctx.stderr;
        
        match output::write_to_output(stdout, content.as_str()) {
            Ok(_) => return 0,
            Err(e) => {
                let content_error = format!("Error writing to output: {}", e);
                match output::write_to_output(stderr, content_error.as_str()) {
                    Ok(_) => return 1,
                    Err(e) => {
                        eprintln!("Error writing to error output: {}", e);
                        return 1;
                    }           
                }
            }
        }
    }
}

