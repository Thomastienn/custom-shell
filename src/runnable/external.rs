use crate::runnable::{ExecContext, RunResult, Runnable};
use crate::utils::output;
use std::process::Command;

pub struct ExternalCommand {
    name: String,
    pub full_path: String,
}

impl ExternalCommand {
    pub fn new(name: String, full_path: String) -> Self {
        ExternalCommand { 
            name,
            full_path
        }
    }
}

impl Runnable for ExternalCommand {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn run(&self, ctx: ExecContext) -> RunResult {
        let args = &ctx.own_parsed_command.args;
        let p_stdout = &ctx.own_parsed_command.stdout;
        let p_stderr = &ctx.own_parsed_command.stderr;

        let stdout = match output::output_to_stdio(p_stdout) {
            Ok(stdout) => stdout,
            Err(e) => {
                eprintln!("Error setting up stdout: {}", e);
                return RunResult::exit(1);
            }
        };
        let stderr = match output::output_to_stdio(p_stderr) {
            Ok(stderr) => stderr,
            Err(e) => {
                eprintln!("Error setting up stderr: {}", e);
                return RunResult::exit(1);
            }
        };
        RunResult::exit(Command::new(&self.name)
            .args(args)
            .stdout(stdout)
            .stderr(stderr)
            .status()
            .map(|s| s.code().unwrap_or(1))
            .unwrap_or(1))
    }

    fn is_builtin(&self) -> bool {
        false
    }

    fn full_path(&self) -> Option<&str> {
        Some(self.full_path.as_str())
    }
}

