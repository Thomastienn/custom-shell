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

#[derive(Debug)]
pub struct ParsedShell {
    pub commands: Vec<ParsedCommand>,
    pub command_buffer: String,
    pub args_buffer: Vec<String>,
    pub background: bool,
}

pub fn parse(tokens: Vec<Token>, strict: bool) -> Result<ParsedShell, String> {
    let mut command: Option<String> = None;
    let mut args = Vec::new();

    let mut stdout = Output::Stdout;
    let mut stderr = Output::Stderr;

    let mut background = false;

    let mut i = 0;

    let mut commands = Vec::new();

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

            Token::Pipe => {
                let final_command = match command {
                    Some(c) => c,
                    None if strict => return Err("empty command".to_string()),
                    _ => "".to_string(),
                };
                let parsed = ParsedCommand {
                    command: final_command,
                    args: args.clone(),
                    stdout: stdout,
                    stderr: stderr,
                };
                commands.push(parsed);

                args.clear();
                stdout = Output::Stdout;
                stderr = Output::Stderr;
                command = None;

                i += 1;
            }
            
            Token::Background => {
                background = true;
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
                let file = expect_file(&tokens, i, &op_str, strict)?;

                let output = match op {
                    RedirectOp::Write => Output::File(file),
                    RedirectOp::Append => Output::AppendFile(file),
                    RedirectOp::Read => unreachable!(),
                };

                match fd.unwrap_or(1) {
                    1 => stdout = output,
                    2 => stderr = output,
                    n if strict => return Err(format!("unsupported file descriptor: {}", n)),
                    _ => ()
                }

                i += 2;
            }
        }
    }

    let command_buffer = match command {
        Some(c) => c,
        None if strict => return Err("empty command".to_string()),
        _ => "".to_string(),
    };

    Ok(ParsedShell {
        commands: commands,
        background: background,
        command_buffer: command_buffer,
        args_buffer: args,
    })

}

fn expect_file(tokens: &[Token], redirect_pos: usize, op: &str, strict: bool) -> Result<String, String> {
    match tokens.get(redirect_pos + 1) {
        Some(Token::Word(file)) => Ok(file.clone()),
        Some(_) if strict => Err(format!("expected file after '{}'", op)),
        None if strict => Err(format!("expected file after '{}'", op)),
        _ => Ok(format!("unknown_redirect_{}", redirect_pos))
    }
}
