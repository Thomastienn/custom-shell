pub mod cd;
pub mod echo;
pub mod exit;
pub mod pwd;
pub mod r#type;

use crate::parser::ParsedCommand;
use crate::utils::output;
use crate::utils::output::Output;
use crate::utils::path::find_executable;
use std::collections::HashMap;
use std::fs::File;
use std::process::{Command, Stdio};

type CommandMap = HashMap<&'static str, Box<dyn Runnable>>;

#[derive(Clone)]
pub struct CommandContext<'a> {
    pub commands: &'a CommandMap,
    pub parsed_command: &'a ParsedCommand,
}

pub trait Runnable {
    fn name(&self) -> &'static str;
    fn run(&self, args: &Vec<String>, ctx: CommandContext) -> i32;
    fn is_builtin(&self) -> bool {
        true
    }
}

pub fn get_commands() -> CommandMap {
    HashMap::from([
        (echo::Echo.name(), Box::new(echo::Echo) as Box<dyn Runnable>),
        (exit::Exit.name(), Box::new(exit::Exit) as Box<dyn Runnable>),
        (
            r#type::Type.name(),
            Box::new(r#type::Type) as Box<dyn Runnable>,
        ),
        (pwd::Pwd.name(), Box::new(pwd::Pwd) as Box<dyn Runnable>),
        (cd::Cd.name(), Box::new(cd::Cd) as Box<dyn Runnable>),
    ])
}

pub fn dispatch(ctx: CommandContext) -> i32 {
    let commands = ctx.commands;
    let p_stdout = &ctx.parsed_command.stdout;
    let p_stderr = &ctx.parsed_command.stderr;
    let command = &ctx.parsed_command.command;
    let args = &ctx.parsed_command.args;

    let stdout = match p_stdout {
        Output::Stdout => Stdio::inherit(),
        Output::Stderr => Stdio::inherit(),
        Output::File(path) => Stdio::from(File::create(path).unwrap()),
        Output::AppendFile(path) => Stdio::from(
            File::options()
                .append(true)
                .create(true)
                .open(path)
                .unwrap(),
        ),
    };

    let stderr = match p_stderr {
        Output::Stdout => Stdio::inherit(),
        Output::Stderr => Stdio::inherit(),
        Output::File(path) => Stdio::from(File::create(path).unwrap()),
        Output::AppendFile(path) => Stdio::from(
            File::options()
                .append(true)
                .create(true)
                .open(path)
                .unwrap(),
        ),
    };

    // Builtin
    if let Some(cmd) = commands.get(command.as_str()) {
        return cmd.run(args, ctx);
    }

    // External
    if let Some(path) = find_executable(command.as_str()) {
        let command_name = path.split('/').last().unwrap_or(command.as_str());
        return Command::new(command_name)
            .args(args)
            .stdout(stdout)
            .stderr(stderr)
            .status()
            .map(|s| s.code().unwrap_or(1))
            .unwrap_or(1);
    }

    let content_error = format!("{}: command not found", command);
    match output::write_to_output(p_stdout, content_error.as_str()) {
        Ok(_) => return 127,
        Err(e) => {
            eprintln!("Error writing to output: {}", e);
            return 1;
        }
    }
}
