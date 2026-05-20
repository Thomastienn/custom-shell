use crate::runnable::{CommandContext, Runnable};
use crate::utils::output;

pub struct Echo;

impl Runnable for Echo {
    fn name(&self) -> &'static str {
        "echo"
    }

    fn run(&self, args: &[&str], _ctx: &CommandContext) -> i32 {
        let content = args.join(" ");
        let stdout = &_ctx.stdout;
        match output::write_to_output(stdout, content.as_str()) {
            Ok(_) => return 0,
            Err(e) => {
                eprintln!("Error writing to output: {}", e);
                return 1;
            }
        }
    }
}
