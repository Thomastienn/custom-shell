use crate::runnable::{CommandContext, Runnable};
use crate::utils::output;

pub struct Complete;

impl Runnable for Complete {
    fn name(&self) -> String {
        "complete".to_string()
    }

    fn run(&self, args: &Vec<String>, ctx: CommandContext) -> i32 {
        0
    }
}

