use crate::runnable::{ExecContext, RunResult, Runnable};
use crate::utils::io;

pub struct History;

impl Runnable for History {
    fn name(&self) -> String {
        "history".to_string()
    }

    fn run(&self, ctx: ExecContext) -> RunResult {
        let stdout = &ctx.own_parsed_command.stdout;
        let content = ctx.shell_ctx.history.iter().enumerate()
            .map(|(i, cmd)| format!("\t{}  {}", i + 1, cmd))
            .collect::<Vec<String>>()
            .join("\n");

        return io::write(content.as_str(), stdout);
    }
}

