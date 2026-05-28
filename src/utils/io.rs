use std::fs::{self, OpenOptions};
use std::io;
use std::io::{Read, Write};
use std::path::Path;
use std::process::{Child, ChildStdout, Stdio};

use crate::runnable::RunResult;

#[derive(Debug, Clone)]
pub enum Input {
    Stdin,
    Pipe,
    File(String),
}

#[derive(Debug, Clone)]
pub enum Output {
    Pipe,
    Stdout,
    Stderr,
    File(String),
    AppendFile(String),
}

#[derive(Debug)]
pub enum PipeInput {
    FromProcess(ChildStdout),
    FromBuiltin(String),
}

#[derive(Debug)]
pub enum PipeOutput {
    Process(Child),
    Text(String),
}

fn create_parent_folder(path: &str) -> std::io::Result<()> {
    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}

pub fn input_to_stdio(input: &Input, pipe_input: &mut Option<PipeInput>) -> io::Result<Stdio> {
    match input {
        Input::Stdin => Ok(Stdio::inherit()),
        Input::File(path) => {
            let file = OpenOptions::new().read(true).open(path)?;

            Ok(Stdio::from(file))
        }
        Input::Pipe => match pipe_input {
            Some(PipeInput::FromProcess(_)) => {
                let Some(PipeInput::FromProcess(stdout)) = pipe_input.take() else {
                    unreachable!();
                };

                Ok(Stdio::from(stdout))
            }
            Some(PipeInput::FromBuiltin(_)) => Ok(Stdio::piped()),
            None => Err(io::Error::new(
                io::ErrorKind::BrokenPipe,
                "missing pipe input",
            )),
        },
    }
}

pub fn read(input: &Input, pipe_input: Option<PipeInput>) -> io::Result<String> {
    match input {
        Input::Stdin => {
            let mut text = String::new();
            io::stdin().read_to_string(&mut text)?;
            Ok(text)
        }
        Input::File(path) => fs::read_to_string(path),
        Input::Pipe => match pipe_input {
            Some(PipeInput::FromProcess(mut stdout)) => {
                let mut text = String::new();
                stdout.read_to_string(&mut text)?;
                Ok(text)
            }
            Some(PipeInput::FromBuiltin(text)) => Ok(text),
            None => Err(io::Error::new(
                io::ErrorKind::BrokenPipe,
                "missing pipe input",
            )),
        },
    }
}

pub fn feed_pipe_input(
    input: &Input,
    child: &mut Child,
    pipe_input: Option<PipeInput>,
) -> io::Result<()> {
    if !matches!(input, Input::Pipe) {
        return Ok(());
    }

    let Some(PipeInput::FromBuiltin(text)) = pipe_input else {
        return Ok(());
    };

    let Some(mut stdin) = child.stdin.take() else {
        return Err(io::Error::new(
            io::ErrorKind::BrokenPipe,
            "child stdin unavailable",
        ));
    };

    stdin.write_all(text.as_bytes())
}

pub fn output_to_stdio(output: &Output) -> io::Result<Stdio> {
    match output {
        Output::Stdout => Ok(Stdio::inherit()),
        Output::Stderr => Ok(Stdio::inherit()),

        Output::File(path) => {
            create_parent_folder(path)?;
            let file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(path)?;

            Ok(Stdio::from(file))
        }

        Output::AppendFile(path) => {
            create_parent_folder(path)?;
            let file = OpenOptions::new().create(true).append(true).open(path)?;

            Ok(Stdio::from(file))
        }

        Output::Pipe => Ok(Stdio::piped()),
    }
}

fn write_to_output(output: &Output, message: &str, exit_code: i32) -> RunResult {
    match output {
        Output::Pipe => RunResult::pipe_output(format!("{}\n", message), exit_code),
        Output::Stdout => {
            println!("{}", message);
            RunResult::exit(exit_code)
        }
        Output::Stderr => {
            eprintln!("{}", message);
            RunResult::exit(exit_code)
        }
        Output::File(filename) => match create_parent_folder(filename)
            .and_then(|_| fs::write(filename, format!("{}\n", message)))
        {
            Ok(_) => RunResult::exit(exit_code),
            Err(e) => {
                eprintln!("Error writing to output: {}", e);
                RunResult::exit(1)
            }
        },
        Output::AppendFile(filename) => match create_parent_folder(filename).and_then(|_| {
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(filename)?;

            writeln!(file, "{}", message)
        }) {
            Ok(_) => RunResult::exit(exit_code),
            Err(e) => {
                eprintln!("Error writing to output: {}", e);
                RunResult::exit(1)
            }
        },
    }
}

pub fn write(message: &str, output: &Output) -> RunResult {
    write_to_output(output, message, 0)
}

pub fn error(message: &str, output: &Output, error_code: i32) -> RunResult {
    write_to_output(output, message, error_code)
}
