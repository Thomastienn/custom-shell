use crate::runnable::{CommandContext, Runnable};

pub struct Exit;

impl Runnable for Exit {
    fn name(&self) -> String {
        "exit".to_string()
    }

    fn run(&self, _args: &Vec<String>, _ctx: CommandContext) -> i32 {
        std::process::exit(0);
    }
}
