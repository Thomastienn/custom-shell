use crate::runnable::{CommandContext, Runnable};
use crate::utils::output;

use std::collections::HashMap;
use std::path::PathBuf;

pub struct Complete {
    completions_path: HashMap<String, PathBuf>,
}

impl Complete {
    pub fn new() -> Self {
        Complete {
            completions_path: HashMap::new(),
        }
    }
}

impl Runnable for Complete {
    fn name(&self) -> String {
        "complete".to_string()
    }

    fn run(&self, args: &Vec<String>, ctx: CommandContext) -> i32 {
        let stdout = &ctx.parsed_command.stdout;
        let stderr = &ctx.parsed_command.stderr;

        for (i, arg) in args.iter().enumerate() {
            match arg.as_str() {
                "-p" => {
                    if i + 1 >= args.len() {
                        return output::error("missing argument for -p", stderr, 1);
                    }

                    let Some(path) = self.completions_path.get(&args[i + 1]) else {
                        return output::error("no completion found for given command", stderr, 1);
                    };

                    let content = format!("complete -C {} {}", path.display(), args[i + 1]);
                    return output::write(content.as_str(), stdout);
                }
                _ => {}
            }
        }

        0
    }
}
