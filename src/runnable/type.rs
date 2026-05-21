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
        let stdout = &ctx.stdout;
        let stderr = &ctx.stderr;

        if let Some(cmd) = ctx.commands.get(command) {
            if cmd.is_builtin() {
                let content = format!("{} is a shell builtin", command);
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

        if let Some(path) = find_executable(command) {
            let content = format!("{} is {}", command, path);
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

        let content = format!("{}: not found", command);
        match output::write_to_output(stderr, content.as_str()) {
            Ok(_) => return 127,
            Err(e) => {
                eprintln!("Error writing to error output: {}", e);
                return 1;
            }
        }
    }
}
