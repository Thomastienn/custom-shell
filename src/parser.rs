use crate::runnable::declare::ShellVariable;
use crate::utils::io::Input;
use crate::{tokenizer::Token, utils::io};
use crate::tokenizer::{RedirectOp, Word, WordPart};
use io::Output;

#[derive(Debug)]
pub struct ParsedCommand {
    pub command: String,
    pub args: Vec<String>,
    pub stdin: Input,
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

pub struct ParseCtx<'a> {
    pub strict: bool,
    pub shell_vars: &'a ShellVariable
}

fn parse_word(word: &Word, ctx: &ParseCtx) -> Result<String, String> {
    let mut result = String::new();
    for part in &word.parts {
        match part {
            WordPart::Literal(lit) => result.push_str(lit),
            WordPart::Variable(var) => {
                if ctx.strict {
                    if let Some(value) = ctx.shell_vars.get(var) {
                        result.push_str(value);
                    } else {
                        result.push_str("");
                    }
                } else {
                    result.push_str(&format!("${}", var));
                }
            }
        }
    }
    if result.is_empty() && ctx.strict {
        return Err("empty word".to_string());
    }
    Ok(result)
}

pub fn parse(tokens: Vec<Token>, ctx: ParseCtx) -> Result<ParsedShell, String> {
    let mut command: Option<String> = None;
    let mut args = Vec::new();

    let mut stdin = Input::Stdin;
    let mut stdout = Output::Stdout;
    let mut stderr = Output::Stderr;

    let mut background = false;

    let mut i = 0;

    let mut commands = Vec::new();

    while i < tokens.len() {
        match &tokens[i] {
            Token::Str(s) => {
                let parsed = parse_word(s, &ctx);
                match parsed {
                    Ok(p) => {
                        if command.is_none() {
                            command = Some(p);
                        } else {
                            args.push(p);
                        }
                    }
                    Err(_) => {}
                }
                i += 1;
            }

            Token::Pipe => {
                if matches!(stdout, Output::Stdout) {
                    stdout = Output::Pipe;
                }

                let final_command = match command.take() {
                    Some(c) => c,
                    None if ctx.strict => return Err("empty command".to_string()),
                    _ => "".to_string(),
                };
                let parsed = ParsedCommand {
                    command: final_command,
                    args: args.clone(),
                    stdin: stdin.clone(),
                    stdout: stdout.clone(),
                    stderr: stderr.clone(),
                };
                commands.push(parsed);

                args.clear();
                stdout = Output::Stdout;
                stderr = Output::Stderr;
                stdin = Input::Pipe;
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
                let file = expect_file(&tokens, i, &op_str, &ctx)?;

                match op {
                    RedirectOp::Read => {
                        match fd.unwrap_or(0) {
                            0 => stdin = Input::File(file),
                            n if ctx.strict => return Err(format!("unsupported file descriptor: {}", n)),
                            _ => {}
                        }
                    }

                    RedirectOp::Write => {
                        match fd.unwrap_or(1) {
                            1 => stdout = Output::File(file),
                            2 => stderr = Output::File(file),
                            n if ctx.strict => return Err(format!("unsupported file descriptor: {}", n)),
                            _ => {}
                        }
                    }

                    RedirectOp::Append => {
                        match fd.unwrap_or(1) {
                            1 => stdout = Output::AppendFile(file),
                            2 => stderr = Output::AppendFile(file),
                            n if ctx.strict => return Err(format!("unsupported file descriptor: {}", n)),
                            _ => {}
                        }
                    }
                }

                i += 2;
            }
        }
    }

    let command_buffer = command.clone().unwrap_or_default();
    let args_buffer = args.clone();

    match command.take() {
        Some(final_command) => {
            commands.push(ParsedCommand {
                command: final_command,
                args,
                stdin,
                stdout,
                stderr,
            });
        }
        None if ctx.strict => return Err("empty command".to_string()),
        _ => {}
    }

    Ok(ParsedShell {
        commands: commands,
        background: background,
        command_buffer: command_buffer,
        args_buffer: args_buffer,
    })

}

fn expect_file(tokens: &[Token], redirect_pos: usize, op: &str, ctx: &ParseCtx) -> Result<String, String> {
    match tokens.get(redirect_pos + 1) {
        Some(Token::Str(wfile)) => {
            return parse_word(wfile, ctx);
        }
        Some(_) if ctx.strict => Err(format!("expected file after '{}'", op)),
        None if ctx.strict => Err(format!("expected file after '{}'", op)),
        _ => Ok(format!("unknown_redirect_{}", redirect_pos))
    }
}
