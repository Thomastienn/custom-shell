pub struct Type;

use crate::runnable::{CommandContext, Runnable};

impl Runnable for Type {
    fn name(&self) -> &'static str {
        "type"
    }

    fn run(&self, args: &[&str], ctx: CommandContext) -> i32 {
        let command = args[0];
        if let Some(cmd) = ctx.commands.get(command) {
            if cmd.is_builtin() {
                println!("{} is a shell builtin", command);
            } else {
                panic!("{} is not a shell builtin", command);
            }
        } else {
            println!("{}: not found", command);
        }
        0
    }
}
