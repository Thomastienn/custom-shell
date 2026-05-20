pub mod echo;
pub mod exit;
pub mod r#type;
pub mod pwd;
pub mod cd;

use crate::utils::path::{find_executable};
use crate::utils::output::Output;
use std::collections::HashMap;
use std::process::Command;

type CommandMap = HashMap<&'static str, Box<dyn Runnable>>;

pub struct CommandContext<'a> {
    pub commands: &'a CommandMap,
    pub stdout: Output,
}

pub trait Runnable {
    fn name(&self) -> &'static str;
    fn run(&self, args: &[&str], ctx: &CommandContext) -> i32;
    fn is_builtin(&self) -> bool {
        true
    }
}

pub fn get_commands() -> CommandMap {
    HashMap::from([
        (echo::Echo.name(), Box::new(echo::Echo) as Box<dyn Runnable>),
        (exit::Exit.name(), Box::new(exit::Exit) as Box<dyn Runnable>),
        (r#type::Type.name(), Box::new(r#type::Type) as Box<dyn Runnable>),
        (pwd::Pwd.name(), Box::new(pwd::Pwd) as Box<dyn Runnable>),
        (cd::Cd.name(), Box::new(cd::Cd) as Box<dyn Runnable>),
    ])
}

pub fn dispatch(
    ctx: &CommandContext,
    command: &str,
    args: &[&str]
) -> i32 {
    let commands = ctx.commands;

    // Builtin
    if let Some(cmd) = commands.get(command) {
        return cmd.run(args, ctx);
    }

    // External
    if let Some(path) = find_executable(command) {
        let command_name = path.split('/').last().unwrap_or(command);
        return Command::new(command_name)
            .args(args)
            .status()
            .map(|s| s.code().unwrap_or(1))
            .unwrap_or(1);
    }

    println!("{}: command not found", command);
    127
}
