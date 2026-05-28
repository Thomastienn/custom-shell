use std::collections::HashMap;

use crate::{runnable::{ExecContext, RunResult, Runnable}, utils::io};

pub struct Declare;

pub type ShellVariable = HashMap<String, String>;

impl Declare {
    pub fn parse_declare(content: &str) -> Option<(String, String)> {
        let parts: Vec<&str> = content.splitn(2, '=').collect();
        if parts.len() == 2 {
            let name = parts[0].trim().to_string();
            let value = parts[1].trim().to_string();
            Some((name, value))
        } else {
            None
        }
    }
}

impl Runnable for Declare {
    fn name(&self) -> String {
        "declare".to_string()
    }

    fn run(&self, ctx: ExecContext) -> RunResult {
        let args = &ctx.own_parsed_command.args;
        let stdout = &ctx.own_parsed_command.stdout;
        let shell_vars = &mut ctx.shell_ctx.shell_vars;
        
        for (i, arg) in args.iter().enumerate() {
            if !arg.starts_with("-") {
                continue;
            }
            if i + 1 >= args.len() {
                eprintln!("declare: option requires an argument -- '{}'", arg);
                return RunResult::exit(1);
            }

            let next_arg = &args[i + 1];
            match arg.as_str() {
                "-p" => {
                    if let Some(value) = shell_vars.get(next_arg) {
                        let content = format!("declare -- {}=\"{}\"", next_arg, value);
                        io::write(content.as_str(), stdout);
                        return RunResult::exit(0);
                    } else {
                        eprintln!("declare: {}: not found", next_arg);
                        return RunResult::exit(1);
                    }
                }
                _ => {
                    eprintln!("declare: invalid option -- '{}'", arg);
                    return RunResult::exit(1);
                }
            }
        }

        match Self::parse_declare(&args[0]) {
            Some((name, value)) => {
                shell_vars.insert(name, value);
            }
            None => {
                eprintln!("declare: invalid argument '{}'", args[0]);
                return RunResult::exit(1);
            }
        }

        RunResult::exit(0)
    }
}

