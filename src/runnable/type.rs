use crate::runnable::{ExecContext, RunResult, Runnable};
use crate::utils::io;

pub struct Type;

impl Runnable for Type {
    fn name(&self) -> String {
        "type".to_string()
    }

    fn run(&self, ctx: ExecContext) -> RunResult {
        let args = &ctx.own_parsed_command.args;
        let command_check = args[0].as_str();
        let stdout = &ctx.own_parsed_command.stdout;
        let stderr = &ctx.own_parsed_command.stderr;

        if let Some(cmd) = ctx.shell_ctx.commands_map.get(command_check) {
            if cmd.is_builtin() {
                let content = format!("{} is a shell builtin", command_check);
                return io::write(content.as_str(), stdout);
            } else {
                let Some(full_path) = cmd.full_path() else {
                    eprintln!("Error: Command {} does not have a full path", command_check);
                    return RunResult::exit(1);
                };
                let content = format!("{} is {}", command_check, full_path);
                return io::write(content.as_str(), stdout);
            }
        }

        let content = format!("{}: not found", command_check);
        return io::error(content.as_str(), stderr, 127);
    }
}
