#[derive(Clone)]
pub enum Output {
    Stdout,
    File(String),
}

pub fn write_to_output(output: &Output, content: impl AsRef<str>) -> std::io::Result<()> {
    let content = content.as_ref();
    match output {
        Output::Stdout => {
            println!("{}", content);
            Ok(())
        }
        Output::File(filename) => {
            std::fs::write(filename, format!("{}\n", content))
        }
    }
}

