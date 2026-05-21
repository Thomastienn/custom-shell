use crate::{tokenizer::Token, utils::output};
use output::Output;

#[derive(Debug)]
pub struct ParsedCommand {
    pub command: String,
    pub args: Vec<String>,
    pub stdout: Output,
    pub stderr: Output,
}

pub fn parse(tokens: Vec<Token>) -> Result<ParsedCommand, String> {
    let mut command: Option<String> = None;
    let mut args = Vec::new();

    let mut stdout = Output::Stdout;
    let mut stderr = Output::Stderr;

    let mut i = 0;

    while i < tokens.len() {
        match &tokens[i] {
            Token::Word(s) => {
                if command.is_none() {
                    command = Some(s.clone());
                } else {
                    args.push(s.clone());
                }

                i += 1;
            }

            Token::RedirectStdout(op) => {
                let file = expect_file(&tokens, i, op)?;
                stdout = Output::File(file);
                i += 2;
            }

            Token::RedirectStderr(op) => {
                let file = expect_file(&tokens, i, op)?;
                stderr = Output::File(file);
                i += 2;
            }
        }
    }

    let command = match command {
        Some(c) => c,
        None => return Err("empty command".to_string()),
    };

    Ok(ParsedCommand {
        command,
        args,
        stdout,
        stderr,
    })
}

fn expect_file(tokens: &[Token], redirect_pos: usize, op: &str) -> Result<String, String> {
    match tokens.get(redirect_pos + 1) {
        Some(Token::Word(file)) => Ok(file.clone()),
        Some(_) => Err(format!("expected file after '{}'", op)),
        None => Err(format!("expected file after '{}'", op)),
    }
}
