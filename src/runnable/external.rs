use crate::runnable::{ExecContext, RunResult, Runnable};
use crate::utils::io::{self, Output};
use std::io::Write;
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
        let p_stdin = &ctx.own_parsed_command.stdin;

        let mut stdin_text = None;
        let stdin = match io::input_to_stdio(p_stdin, ctx.pipe_input, &mut stdin_text) {
            Ok(stdin) => stdin,
            Err(e) => {
                eprintln!("Error setting up stdin: {}", e);
                return RunResult::exit(1);
            }
        };

        let stdout = match io::output_to_stdio(p_stdout) {
            Ok(stdout) => stdout,
            Err(e) => {
                eprintln!("Error setting up stdout: {}", e);
                return RunResult::exit(1);
            }
        };
        let stderr = match io::output_to_stdio(p_stderr) {
            Ok(stderr) => stderr,
            Err(e) => {
                eprintln!("Error setting up stderr: {}", e);
                return RunResult::exit(1);
            }
        };
        let child = Command::new(&self.name)
            .args(args)
            .stdin(stdin)
            .stdout(stdout)
            .stderr(stderr)
            .spawn();

        let mut child = match child {
            Ok(child) => child,
            Err(_) => return RunResult::exit(1),
        };

        if let Some(text) = stdin_text {
            let Some(mut child_stdin) = child.stdin.take() else {
                eprintln!("Error writing to stdin: child stdin unavailable");
                return RunResult::exit(1);
            };
            if let Err(e) = child_stdin.write_all(text.as_bytes()) {
                eprintln!("Error writing to stdin: {}", e);
                return RunResult::exit(1);
            }
        }

        if matches!(p_stdout, Output::Pipe) {
            return RunResult::pipe_process(child);
        }

        RunResult::exit(child.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1))
    }

    fn is_builtin(&self) -> bool {
        false
    }

    fn full_path(&self) -> Option<&str> {
        Some(self.full_path.as_str())
    }
}
