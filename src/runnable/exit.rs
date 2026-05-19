pub struct Exit;

use crate::runnable::{CommandContext, Runnable};

impl Runnable for Exit {
    fn name(&self) -> &'static str {
        "exit"
    }

    fn run(&self, _args: &[&str], _ctx: CommandContext) -> i32 {
        std::process::exit(0);
    }
}
