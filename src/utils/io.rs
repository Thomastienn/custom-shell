use std::fs::{OpenOptions, self};
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

pub fn write_to_output(output: &Output, content: impl AsRef<str>) -> std::io::Result<()> {
    let content = content.as_ref();
    match output {
        Output::Stdout => {
            println!("{}", content);
            Ok(())
        }
        Output::Stderr => {
            eprintln!("{}", content);
            Ok(())
        }
        Output::File(filename) => {
            create_parent_folder(filename)?;
            std::fs::write(filename, format!("{}\n", content))
        }
        Output::AppendFile(filename) => {
            create_parent_folder(filename)?;
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(filename)?;

            writeln!(file, "{}", content)
        }
        Output::Pipe => Ok(()),
    }
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

pub fn write(message: &str, output: &Output) -> RunResult {
    if matches!(output, Output::Pipe) {
        return RunResult::pipe_output(format!("{}\n", message), 0);
    }

    match write_to_output(output, message) {
        Ok(_) => return RunResult::exit(0),
        Err(e) => {
            eprintln!("Error writing to output: {}", e);
            return RunResult::exit(1);
        }
    }
}

pub fn error(message: &str, output: &Output, error_code: i32) -> RunResult {
    if matches!(output, Output::Pipe) {
        return RunResult::pipe_output(format!("{}\n", message), error_code);
    }

    match write_to_output(output, message) {
        Ok(_) => return RunResult::exit(error_code),
        Err(e) => {
            eprintln!("Error writing to output: {}", e);
            return RunResult::exit(1);
        }
    }
}
