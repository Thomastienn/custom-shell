use crate::runnable::{CommandContext, Runnable };
use crate::utils::output;

pub struct Type;

impl Runnable for Type {
    fn name(&self) -> String {
        "type".to_string()
    }

    fn run(&self, args: &Vec<String>, ctx: CommandContext) -> i32 {
        let command = args[0].as_str();
        let stdout = &ctx.parsed_command.stdout;
        let stderr = &ctx.parsed_command.stderr;

        if let Some(cmd) = ctx.commands.get(command) {
            if cmd.is_builtin() {
                let content = format!("{} is a shell builtin", command);
                return output::write(content.as_str(), stdout);
            } else {
                let Some(full_path) = cmd.full_path() else {
                    eprintln!("Error: Command {} does not have a full path", command);
                    return 1;
                };
                let content = format!("{} is {}", command, full_path);
                return output::write(content.as_str(), stdout);
            }
        }

        let content = format!("{}: not found", command);
        return output::error(content.as_str(), stderr, 127);
    }
}
