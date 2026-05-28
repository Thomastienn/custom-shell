use crate::runnable::{ExecContext, RunResult, Runnable};

pub struct Exit;

impl Runnable for Exit {
    fn name(&self) -> String {
        "exit".to_string()
    }

    fn run(&self, _ctx: ExecContext) -> RunResult {
        RunResult::exit_shell()
    }
}
