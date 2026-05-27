pub mod cd;
pub mod echo;
pub mod exit;
pub mod external;
pub mod pwd;
pub mod r#type;
pub mod complete;
pub mod jobs;
pub mod history;

use crate::parser::{ParsedCommand, ParsedShell};
use crate::runnable::jobs::{JobInfo, Jobs};
use crate::structures::dll::DoublyLinkedList;
use crate::utils::io::{self, PipeOutput, PipeInput};
use crate::structures::trie::{Trie};
use crate::runnable::external::ExternalCommand;
use crate::utils::path::PathUtils;
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Child;

pub type CommandMap = HashMap<String, Box<dyn Runnable>>;
pub type CompletionPath = HashMap<String, PathBuf>;
pub type JobList = DoublyLinkedList<JobInfo>;
pub struct ShellContext<'a> {
    pub commands_map: &'a CommandMap,
    pub completions_path: &'a mut CompletionPath,
    pub file_trie: &'a mut Trie,
    pub job_list: &'a mut JobList,
    pub history: &'a Vec<String>,
}

pub struct ExecContext<'a, 'b> {
    pub shell_ctx: &'a mut ShellContext<'b>,
    pub own_parsed_command: &'a ParsedCommand,
    pub pipe_input: Option<PipeInput>,
}

#[derive(Debug)]
pub struct RunResult {
    pub exit_code: i32,
    pub pipe_output: Option<PipeOutput>,
}

impl RunResult {
    pub fn exit(code: i32) -> Self {
        RunResult {
            exit_code: code,
            pipe_output: None,
        }
    }

    pub fn pipe_process(child: Child) -> Self {
        RunResult {
            exit_code: 0,
            pipe_output: Some(PipeOutput::Process(child)),
        }
    }

    pub fn pipe_output(output: String, code: i32) -> Self {
        RunResult {
            exit_code: code,
            pipe_output: Some(PipeOutput::Text(output)),
        }
    }
}
impl PartialEq for RunResult {
    fn eq(&self, other: &Self) -> bool {
        self.exit_code == other.exit_code
    }
}

pub trait Runnable {
    fn name(&self) -> String;
    fn run(&self, ctx: ExecContext<'_, '_>) -> RunResult;
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
        Box::new(history::History) as Box<dyn Runnable>,
    ];
    for cmd in builtin_cmds {
        cmds.insert(cmd.name(), cmd);
    }

    cmds
}

pub fn dispatch(mut shell_ctx: ShellContext, parsed_cmd: ParsedShell) -> RunResult {
    let commands = shell_ctx.commands_map;
    let background = parsed_cmd.background;
    let mut pipe_input: Option<PipeInput> = None;
    let mut pipeline_children: Vec<Child> = Vec::new();
    let mut last_result = RunResult::exit(0);

    for cmd_process in &parsed_cmd.commands {
        let stdout = &cmd_process.stdout;
        let stderr = &cmd_process.stderr;
        let command = &cmd_process.command;
        
        // just create the file
        let _ = io::output_to_stdio(stdout);
        let _ = io::output_to_stdio(stderr);

        let exe_ctx = ExecContext {
            shell_ctx: &mut shell_ctx,
            own_parsed_command: cmd_process,
            pipe_input: pipe_input.take(),
        };
        if background {
            last_result = Jobs.run_background(exe_ctx);
            continue;
        }

        if let Some(cmd) = commands.get(command) {
            let mut result = cmd.run(exe_ctx);

            match result.pipe_output.take() {
                Some(PipeOutput::Process(mut child)) => {
                    pipe_input = child
                        .stdout
                        .take()
                        .map(PipeInput::FromProcess);
                    pipeline_children.push(child);
                }
                Some(PipeOutput::Text(output)) => {
                    pipe_input = Some(PipeInput::FromBuiltin(output));
                }
                None => pipe_input = None,
            }

            last_result = result;
            continue;
        }

        let content_error = format!("{}: command not found", command);
        eprintln!("{}", content_error);
        for mut child in pipeline_children {
            let _ = child.wait();
        }
        return RunResult::exit(127);
    }

    for mut child in pipeline_children {
        let _ = child.wait();
    }

    return last_result;
}
