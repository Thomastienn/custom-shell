use crate::runnable::{CommandContext, Runnable};
use crate::utils::output;

pub struct Echo;

impl Runnable for Echo {
    fn name(&self) -> String {
        "echo".to_string()
    }

    fn run(&self, args: &Vec<String>, ctx: CommandContext) -> i32 {
        let content = args.join(" ");
        let stdout = &ctx.parsed_command.stdout;
        // let stderr = &ctx.parsed_command.stderr;

        return output::write(content.as_str(), stdout);
    }
}
