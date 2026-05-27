use std::fs::{self, OpenOptions};
use std::io;
use std::io::Write;
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
