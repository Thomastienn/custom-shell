use crate::runnable::{CommandContext, Runnable};
use crate::utils::output;

pub struct Complete;

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
                        let err_msg = format!("complete: {}: no completion specification", flag_arg);
                        return output::error(err_msg.as_str(), stderr, 1);
                    };

                    let content = format!("complete -C {} {}", path.display(), flag_arg);
                    return output::write(content.as_str(), stdout);
                }
                "-C" => {
                    if i + 2 >= args.len() {
                        let err_msg = format!("complete: option {} requires 2 arguments", arg);
                        return output::error(err_msg.as_str(), stderr, 1);
                    }
                    let name_exe = &args[i + 2];
                    completions_path.insert(flag_arg.clone(), name_exe.clone().into());
                }
                _ => {}
            }
        }

        0
    }
}
