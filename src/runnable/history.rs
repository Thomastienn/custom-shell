use std::fs;

use crate::runnable::{ExecContext, RunResult, Runnable};
use crate::utils::io;

pub struct History;

impl Runnable for History {
    fn name(&self) -> String {
        "history".to_string()
    }

    fn run(&self, ctx: ExecContext) -> RunResult {
        let args = &ctx.own_parsed_command.args;
        let stdout = &ctx.own_parsed_command.stdout;

        for (i, arg) in args.iter().enumerate() {
            if !arg.starts_with("-") {
                continue;
            }
            if i + 1 >= args.len() {
                eprintln!("Error: Option {} requires an argument", arg);
                return RunResult::exit(1);
            }
            let next_arg = &args[i + 1];
            match arg.as_str() {
                "-r" => {
                    let Ok(content) = fs::read_to_string(next_arg) else {
                        eprintln!("Error: Failed to read file {}", next_arg);
                        return RunResult::exit(1);
                    };
                    content.lines().for_each(|line| ctx.shell_ctx.history.push(line.to_string()));
                    return RunResult::exit(0);
                }

                "-w" => {
                    let content = ctx.shell_ctx.history.join("\n");
                    if let Err(e) = fs::write(next_arg, content) {
                        eprintln!("Error: Failed to write to file {}: {}", next_arg, e);
                        return RunResult::exit(1);
                    }
                    return RunResult::exit(0);
                }
                _ => {
                    eprintln!("Error: Unknown option {}", arg);
                    return RunResult::exit(1);
                }
            }
        }

        let mut start = 0;
        if args.len() > 0 {
            if let Ok(n) = args[0].parse::<usize>() {
                start = ctx.shell_ctx.history.len().saturating_sub(n);
            }
        }
        
        let content = ctx.shell_ctx.history.iter().enumerate()
            .skip(start)
            .map(|(i, cmd)| format!("\t{}  {}", i + 1, cmd))
            .collect::<Vec<String>>()
            .join("\n");

        return io::write(content.as_str(), stdout);
    }
}

