#[derive(Clone, Debug)]
pub enum Output {
    Stdout,
    Stderr,
    File(String),
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
    }
}

