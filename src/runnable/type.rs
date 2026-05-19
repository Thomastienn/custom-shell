use crate::runnable::{CommandContext, Runnable };
use crate::utils::path::{find_executable};

pub struct Type;

impl Runnable for Type {
    fn name(&self) -> &'static str {
        "type"
    }

    fn run(&self, args: &[&str], ctx: CommandContext) -> i32 {
        let command = args[0];
        if let Some(cmd) = ctx.commands.get(command) {
            if cmd.is_builtin() {
                println!("{} is a shell builtin", command);
                return 0;
            }
        }
        if let Some(path) = find_executable(command) {
            println!("{} is {}", command, path);
            return 0;
        }
        println!("{}: not found", command);
        0
    }
}
