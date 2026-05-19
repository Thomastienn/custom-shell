use crate::runnable::{CommandContext, Runnable};

pub struct Echo;

impl Runnable for Echo {
    fn name(&self) -> &'static str {
        "echo"
    }

    fn run(&self, args: &[&str], _ctx: CommandContext) -> i32 {
        println!("{}", args.join(" "));
        0
    }
}
