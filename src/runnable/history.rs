use crate::runnable::{ExecContext, RunResult, Runnable};
use crate::utils::io;

pub struct History;

impl Runnable for History {
    fn name(&self) -> String {
        "history".to_string()
    }

    fn run(&self, ctx: ExecContext) -> RunResult {
        let args = &ctx.own_parsed_command.args;
        let stdout = &ctx.own_parsed_command.stdout;

        let mut start = 0;
        if args.len() > 0 {
            if let Ok(n) = args[0].parse::<usize>() {
                start = ctx.shell_ctx.history.len().saturating_sub(n);
            }
        }
        
        let content = ctx.shell_ctx.history.iter().enumerate()
            .skip(start)
            .map(|(i, cmd)| format!("\t{}  {}", i + 1, cmd))
            .collect::<Vec<String>>()
            .join("\n");

        return io::write(content.as_str(), stdout);
    }
}

