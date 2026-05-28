use std::fs;

use crate::runnable::{ExecContext, RunResult, Runnable};
use crate::utils::io::{self, Output};

pub struct History;

impl History {
    pub fn read_and_load(path: &str, entries: &mut Vec<String>) -> RunResult {
        let content = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error reading file {}: {}", path, e);
                return RunResult::exit(1);
            }
        };
        content
            .lines()
            .for_each(|line| entries.push(line.to_string()));
        return RunResult::exit(0);
    }
}

impl Runnable for History {
    fn name(&self) -> String {
        "history".to_string()
    }

    fn run(&self, ctx: ExecContext) -> RunResult {
        let args = &ctx.own_parsed_command.args;
        let stdout = &ctx.own_parsed_command.stdout;
        let history = &mut ctx.shell_ctx.history;
        let history_entries = &mut history.entries;

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
                    return Self::read_and_load(next_arg, history_entries);
                }

                "-w" => {
                    let content = history_entries.join("\n");
                    let write_type = Output::File(next_arg.clone());
                    return io::write(content.as_str(), &write_type);
                }
                "-a" => {
                    let content = history_entries[history.last_appended..].join("\n");
                    let append_type = Output::AppendFile(next_arg.clone());

                    history.last_appended = history_entries.len();
                    return io::write(content.as_str(), &append_type);
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
                start = history_entries.len().saturating_sub(n);
            }
        }

        let content = history_entries
            .iter()
            .enumerate()
            .skip(start)
            .map(|(i, cmd)| format!("\t{}  {}", i + 1, cmd))
            .collect::<Vec<String>>()
            .join("\n");

        return io::write(content.as_str(), stdout);
    }
}
