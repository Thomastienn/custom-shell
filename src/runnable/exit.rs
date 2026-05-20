use crate::runnable::{CommandContext, Runnable};

pub struct Exit;

impl Runnable for Exit {
    fn name(&self) -> &'static str {
        "exit"
    }

    fn run(&self, _args: &[&str], _ctx: &CommandContext) -> i32 {
        std::process::exit(0);
    }
}
