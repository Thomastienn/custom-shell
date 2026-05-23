use crate::runnable::{CommandContext, Runnable};
use crate::utils::output;

pub struct Jobs;

impl Runnable for Jobs {
    fn name(&self) -> String {
        "jobs".to_string()
    }

    fn run(&self, args: &Vec<String>, ctx: CommandContext) -> i32 {
        0
    }
}
