use std::path::PathBuf;
use std::process::Command;

use crate::runnable::{CommandContext, Runnable};
use crate::utils::output;

pub struct Complete;

impl Complete {
    pub fn get_completion_spec(
        name_exe: &str,
        partial: &str,
        previous: &str,
        path: &PathBuf,
        buffer: &str,
        cursor_pos: usize,
    ) -> Vec<String> {
        let args = vec![
            name_exe.to_string(),
            partial.to_string(),
            previous.to_string(),
        ];
        let output = Command::new(path)
            .args(args)
            .env("COMP_LINE", buffer)
            .env("COMP_POINT", cursor_pos.to_string())
            .output();
        match output {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                stdout.lines().map(|line| line.to_string()).collect()
            }
            Err(e) => {
                eprintln!(
                    "Error: Failed to execute completion command at path {}: {}",
                    path.display(),
                    e
                );
                vec![]
            }
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
        let completions_path = ctx.completions_path;

        for (i, arg) in args.iter().enumerate() {
            if !arg.starts_with("-") {
                continue;
            }
            if i + 1 >= args.len() {
                let err_msg = format!("complete: option {} requires an argument", arg);
                return output::error(err_msg.as_str(), stderr, 1);
            }
            let flag_arg = &args[i + 1];
            match arg.as_str() {
                "-p" => {
                    let Some(path) = completions_path.get(flag_arg) else {
                        let err_msg =
                            format!("complete: {}: no completion specification", flag_arg);
                        return output::error(err_msg.as_str(), stderr, 1);
                    };

                    let content = format!("complete -C '{}' {}", path.display(), flag_arg);
                    return output::write(content.as_str(), stdout);
                }
                "-C" => {
                    if i + 2 >= args.len() {
                        let err_msg = format!("complete: option {} requires 2 arguments", arg);
                        return output::error(err_msg.as_str(), stderr, 1);
                    }
                    let name_exe = &args[i + 2];
                    let path_buf = PathBuf::from(flag_arg);
                    // Complete::add_completion_spec(completions_trie, name_exe, &path_buf);
                    completions_path.insert(name_exe.clone(), path_buf);
                }
                _ => {}
            }
        }

        0
    }
}
