use std::fs::OpenOptions;
use std::io;
use std::process::Stdio;
use std::io::Write;

#[derive(Clone, Debug)]
pub enum Output {
    Stdout,
    Stderr,
    File(String),
    AppendFile(String),
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
            std::fs::write(filename, format!("{}\n", content))
        }
        Output::AppendFile(filename) => {
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
            let file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(path)?;

            Ok(Stdio::from(file))
        }

        Output::AppendFile(path) => {
            let file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)?;

            Ok(Stdio::from(file))
        }
    }
}
