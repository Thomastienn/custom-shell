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
        
        match output::write_to_output(stdout, content.as_str()) {
            Ok(_) => return 0,
            Err(e) => {
                eprintln!("Error writing to output: {}", e);
                return 1;
            }
        }
    }
}
