use crate::runnable::{ExecContext, RunResult, Runnable};
use crate::utils::io;

pub struct Echo;

impl Runnable for Echo {
    fn name(&self) -> String {
        "echo".to_string()
    }

    fn run(&self, ctx: ExecContext) -> RunResult {
        let args = &ctx.own_parsed_command.args;
        let stdout = &ctx.own_parsed_command.stdout;
        
        let content = args.join(" ");
        // let stderr = &ctx.parsed_command.stderr;

        return io::write(content.as_str(), stdout);
    }
}
