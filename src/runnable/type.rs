use crate::runnable::{CommandContext, Runnable };
use crate::utils::path::{find_executable};
use crate::utils::output;

pub struct Type;

impl Runnable for Type {
    fn name(&self) -> &'static str {
        "type"
    }

    fn run(&self, args: &[&str], ctx: &CommandContext) -> i32 {
        let command = args[0];
        let output = &ctx.stdout;

        if let Some(cmd) = ctx.commands.get(command) {
            if cmd.is_builtin() {
                let content = format!("{} is a shell builtin", command);
                match output::write_to_output(output, content.as_str()) {
                    Ok(_) => return 0,
                    Err(e) => {
                        eprintln!("Error writing to output: {}", e);
                        return 1;
                    }
                }
            }
        }
        if let Some(path) = find_executable(command) {
            let content = format!("{} is {}", command, path);
            match output::write_to_output(output, content.as_str()) {
                Ok(_) => return 0,
                Err(e) => {
                    eprintln!("Error writing to output: {}", e);
                    return 1;
                }
            }
        }
        eprintln!("{}: not found", command);
        0
    }
}
