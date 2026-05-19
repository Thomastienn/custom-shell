use crate::runnable::{CommandContext, Runnable };
use std::env;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

pub struct Type;

impl Runnable for Type {
    fn name(&self) -> &'static str {
        "type"
    }

    fn run(&self, args: &[&str], ctx: CommandContext) -> i32 {
        let command = args[0];
        if let Some(cmd) = ctx.commands.get(command) {
            if cmd.is_builtin() {
                println!("{} is a shell builtin", command);
                return 0;
            }
        }
        let path_res = env::var("PATH");
        if let Ok(path_str) = path_res {
            let exec_path = path_str.split(':').find(|path| {
                let full_path = format!("{}/{}", path, command);
                let file_path = Path::new(&full_path);

                file_path.exists() && file_path.metadata().unwrap().permissions().mode() & 0o111 != 0
            });
            if let Some(exec_path) = exec_path {
                println!("{} is {}", command, exec_path);
                return 0;
            }
        }
        println!("{}: not found", command);
        0
    }
}
