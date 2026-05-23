use std::fs::{OpenOptions, self};
use std::io;
use std::process::Stdio;
use std::io::Write;
use std::path::Path;

#[derive(Clone, Debug)]
pub enum Output {
    Stdout,
    Stderr,
    File(String),
    AppendFile(String),
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
            let file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)?;

            Ok(Stdio::from(file))
        }
    }
}

pub fn write(message: &str, output: &Output) -> i32 {
    match write_to_output(output, message) {
        Ok(_) => return 0,
        Err(e) => {
            eprintln!("Error writing to output: {}", e);
            return 1;
        }
    }
}

pub fn error(message: &str, output: &Output, error_code: i32) -> i32 {
    match write_to_output(output, message) {
        Ok(_) => return error_code,
        Err(e) => {
            eprintln!("Error writing to output: {}", e);
            return 1;
        }
    }
}
