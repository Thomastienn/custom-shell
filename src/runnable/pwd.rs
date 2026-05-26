use crate::runnable::{ExecContext, RunResult, Runnable};
use crate::utils::io;
use std::env;

pub struct Pwd;

impl Runnable for Pwd {
    fn name(&self) -> String {
        "pwd".to_string()
    }

    fn run(&self, ctx: ExecContext) -> RunResult {
        let content = env::current_dir().unwrap().display().to_string();
        let stdout = &ctx.own_parsed_command.stdout;
        // let stderr = &ctx.parsed_command.stderr;

        return io::write(content.as_str(), stdout);
    }
}

