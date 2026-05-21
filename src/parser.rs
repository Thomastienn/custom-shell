use crate::{tokenizer::Token, utils::output};
use crate::tokenizer::RedirectOp;
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

            Token::Redirect { fd, op } => {
                let op_str = format!(
                    "{}{}",
                    fd.map(|f| f.to_string()).unwrap_or_default(),
                    match op {
                        RedirectOp::Read => "<",
                        RedirectOp::Write => ">",
                        RedirectOp::Append => ">>",
                    }
                );
                let file = expect_file(&tokens, i, &op_str)?;

                let output = match op {
                    RedirectOp::Write => Output::File(file),
                    RedirectOp::Append => Output::AppendFile(file),
                    RedirectOp::Read => unreachable!(),
                };

                match fd.unwrap_or(1) {
                    1 => stdout = output,
                    2 => stderr = output,
                    n => return Err(format!("unsupported file descriptor: {}", n)),
                }

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
