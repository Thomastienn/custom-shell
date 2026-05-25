pub mod cd;
pub mod echo;
pub mod exit;
pub mod external;
pub mod pwd;
pub mod r#type;
pub mod complete;
pub mod jobs;

use crate::parser::ParsedCommand;
use crate::runnable::jobs::{JobInfo, Jobs};
use crate::structures::dll::DoublyLinkedList;
use crate::utils::output;
use crate::structures::trie::{Trie};
use crate::runnable::external::ExternalCommand;
use crate::utils::path::PathUtils;
use std::collections::HashMap;
use std::path::PathBuf;

pub type CommandMap = HashMap<String, Box<dyn Runnable>>;
pub type CompletionPath = HashMap<String, PathBuf>;
pub type JobList = DoublyLinkedList<JobInfo>;
pub struct CommandContext<'a> {
    pub commands: &'a CommandMap,
    pub completions_path: &'a mut CompletionPath,
    pub parsed_command: &'a ParsedCommand,
    pub file_trie: &'a mut Trie,
    pub job_list: &'a mut JobList,
}

pub trait Runnable {
    fn name(&self) -> String;
    fn run(&self, args: &Vec<String>, ctx: CommandContext) -> i32;
    fn is_builtin(&self) -> bool {
        true
    }
    fn full_path(&self) -> Option<&str> {
        None
    }
}

pub fn get_commands() -> CommandMap {
    let mut cmds = HashMap::new();

    let executables = PathUtils::all_executables_in_path();

    for exe in executables {
        let Some(exe_name) = PathUtils::get_filename(&exe) else {
            continue;
        };
        let Some(full_path) = PathUtils::get_fullpath(&exe) else {
            continue;
        };
        cmds.insert(
            exe_name.clone(),
            Box::new(ExternalCommand::new(exe_name, full_path)) as Box<dyn Runnable>,
        );
    }

    let builtin_cmds = [
        Box::new(cd::Cd) as Box<dyn Runnable>,
        Box::new(echo::Echo) as Box<dyn Runnable>,
        Box::new(exit::Exit) as Box<dyn Runnable>,
        Box::new(pwd::Pwd) as Box<dyn Runnable>,
        Box::new(r#type::Type) as Box<dyn Runnable>,
        Box::new(complete::Complete) as Box<dyn Runnable>,
        Box::new(jobs::Jobs) as Box<dyn Runnable>,
    ];
    for cmd in builtin_cmds {
        cmds.insert(cmd.name(), cmd);
    }

    cmds
}

pub fn dispatch(ctx: CommandContext) -> i32 {
    let commands = ctx.commands;
    let command = &ctx.parsed_command.command;
    let args = &ctx.parsed_command.args;

    let stdout = &ctx.parsed_command.stdout;
    let stderr = &ctx.parsed_command.stderr;

    // just create the file
    let _ = output::output_to_stdio(stdout);
    let _ = output::output_to_stdio(stderr);

    if ctx.parsed_command.background {
        return Jobs.run_background(args, ctx);
    }

    if let Some(cmd) = commands.get(command) {
        return cmd.run(args, ctx);
    }

    let content_error = format!("{}: command not found", command);
    eprintln!("{}", content_error);
    127
}
