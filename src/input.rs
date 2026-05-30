use std::collections::VecDeque;
use std::io::{self, ErrorKind, Write};
use std::path::PathBuf;

use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use crossterm::terminal;

use crate::parser::{self, ParseCtx, ParsedShell};
use crate::runnable::CommandMap;
use crate::runnable::complete::{Complete, CompletionPath};
use crate::runnable::declare::ShellVariable;
use crate::runnable::history::HistoryCtx;
use crate::structures::string;
use crate::structures::trie::Trie;
use crate::tokenizer::{LexedToken, Tokenizer};
use crate::utils::color;

struct RawModeGuard;

impl RawModeGuard {
    fn new() -> io::Result<Self> {
        terminal::enable_raw_mode()?;
        Ok(Self)
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        let _ = terminal::disable_raw_mode();
    }
}

pub struct InputCtx<'a> {
    pub _commands: &'a CommandMap,
    pub completions_path: &'a CompletionPath,
    pub cmd_pref: &'a Trie,
    pub filesystem_pref: &'a Trie,
    pub history: &'a mut HistoryCtx,
    pub shell_vars: &'a ShellVariable,
}

type PrevArg = String;
type CurrentArg = String;
type PathComplete = String;

#[derive(Debug)]
pub enum SuggestionType {
    Complete(PrevArg, CurrentArg, PathComplete),
    Command,
    Filesystem,
}

pub struct InputShell;

impl InputShell {
    fn tokenize_buffer(buffer: &str) -> Vec<LexedToken> {
        let mut tokenizer = Tokenizer::new(buffer.to_string());
        tokenizer.tokenize()
    }

    fn parse_buffer(
        strict: bool,
        input_ctx: &InputCtx,
        lexed_tokens: &Vec<LexedToken>,
    ) -> Result<ParsedShell, io::Error> {
        let parse_ctx = ParseCtx {
            strict,
            shell_vars: input_ctx.shell_vars,
        };

        let tokens = lexed_tokens.iter().map(|t| &t.token).collect();

        parser::parse(tokens, parse_ctx).map_err(|e| io::Error::new(ErrorKind::InvalidInput, e))
    }

    fn render(
        buffer: &str,
        prompt: &str,
        tokens: &Vec<LexedToken>,
        stdout: &mut io::Stdout,
    ) -> io::Result<()> {
        let mut pos = 0;
        print!("\r{prompt}\x1b[K");
        for lex_tok in tokens {
            print!("{}", &buffer[pos..lex_tok.start]);
            
            let raw = &buffer[lex_tok.start..lex_tok.end];
            print!("{}", color::color_token(raw, &lex_tok));

            pos = lex_tok.end;
        }
        print!("{}", &buffer[pos..]);

        stdout.flush()
    }

    pub fn read_line(prompt: &str, mut ctx: InputCtx) -> Result<ParsedShell, io::Error> {
        let _raw_mode = RawModeGuard::new()?;

        let mut stdout = io::stdout();
        let mut buffer = String::new();

        let mut key_presses: VecDeque<KeyCode> = VecDeque::new();
        const MAX_HISTORY_KEYPRESSES: usize = 3;

        let mut tab_cnt = 0;
        let mut current_history = ctx.history.entries.len(); // 1-indexed since its usize

        print!("{prompt}");
        stdout.flush()?;
        
        loop {
            let Event::Key(key) = event::read()? else {
                continue;
            };

            let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);

            if key_presses.len() == MAX_HISTORY_KEYPRESSES {
                key_presses.pop_front();
            }
            assert!(tab_cnt <= 2);
            if key_presses.back() != Some(&KeyCode::Tab) {
                tab_cnt = 0;
            }
            key_presses.push_back(key.code);

            let mut tokenizer = Tokenizer::new(buffer.to_string());
            let lexed_tokens = tokenizer.tokenize();
            let parsed_cmd = Self::parse_buffer(false, &ctx, &lexed_tokens).unwrap();

            match key.code {
                KeyCode::Char(c) => {
                    if ctrl {
                        match c {
                            'c' => {
                                print!("\r\n");
                                stdout.flush()?;
                                return Err(io::Error::new(ErrorKind::Interrupted, "Interrupted"));
                            }
                            'j' => {
                                return Self::submit(&mut stdout, &buffer, &mut ctx);
                            }
                            'z' => {
                                dbg!(&parsed_cmd);
                            }
                            _ => continue,
                        }
                    }
                    buffer.push(c);
                    Self::render(
                        &buffer,
                        prompt,
                        &Self::tokenize_buffer(&buffer),
                        &mut stdout,
                    )?;
                }

                KeyCode::Up => {
                    if current_history > 0 {
                        current_history -= 1;
                        buffer = ctx.history.entries[current_history].clone();
                        Self::render(
                            &buffer,
                            prompt,
                            &Self::tokenize_buffer(&buffer),
                            &mut stdout,
                        )?;
                    }
                }

                KeyCode::Down => {
                    if current_history + 1 < ctx.history.entries.len() {
                        current_history += 1;
                        buffer = ctx.history.entries[current_history].clone();
                        Self::render(
                            &buffer,
                            prompt,
                            &Self::tokenize_buffer(&buffer),
                            &mut stdout,
                        )?;
                    }
                }

                KeyCode::Backspace => {
                    if buffer.pop().is_some() {
                        print!("\x08 \x08");
                        Self::render(
                            &buffer,
                            prompt,
                            &Self::tokenize_buffer(&buffer),
                            &mut stdout,
                        )?;
                    }
                }

                KeyCode::Tab => {
                    let mut suggestions: Vec<String>;
                    let partial: &str;
                    let cmd_parsed = parsed_cmd.command_buffer.as_str();
                    let cmd_args = &parsed_cmd.args_buffer;

                    let mut autocomplete: Option<SuggestionType> = None;

                    if let Some(path) = ctx.completions_path.get(cmd_parsed) {
                        let mut cur_arg = String::new();
                        let mut prev = cmd_parsed.to_string();
                        if cmd_args.len() >= 2 {
                            prev = cmd_args[cmd_args.len() - 2].clone();
                        }
                        if cmd_args.len() >= 1 {
                            cur_arg = cmd_args.last().unwrap().clone();
                        }
                        let path_str = path.to_str().unwrap().to_string();
                        autocomplete = Some(SuggestionType::Complete(prev, cur_arg, path_str));
                    }

                    let partial_word = cmd_args.len() >= 1;
                    let not_type = buffer.ends_with(' ');
                    autocomplete.get_or_insert_with(|| {
                        if partial_word || not_type {
                            SuggestionType::Filesystem
                        } else {
                            SuggestionType::Command
                        }
                    });

                    let mut last_token = buffer.split_whitespace().last().unwrap_or("");
                    if not_type {
                        last_token = "";
                    }
                    // dbg!(&autocomplete);
                    match autocomplete.unwrap() {
                        SuggestionType::Complete(prev, partial_arg, path) => {
                            suggestions = Complete::get_completion_spec(
                                cmd_parsed,
                                partial_arg.as_str(),
                                prev.as_str(),
                                &PathBuf::from(path),
                                buffer.as_str(),
                                buffer.len(),
                            );
                        }
                        SuggestionType::Command => {
                            suggestions = ctx.cmd_pref.autocomplete(last_token);
                        }
                        SuggestionType::Filesystem => {
                            suggestions = ctx.filesystem_pref.autocomplete(last_token);
                        }
                    }
                    partial = last_token;
                    // dbg!(&suggestions);

                    if suggestions.is_empty() {
                        print!("\x07");
                        stdout.flush()?;
                        continue;
                    }
                    if suggestions.len() == 1 {
                        let suffix = &suggestions[0][partial.len()..];
                        buffer.push_str(suffix);
                        if !suggestions[0].ends_with('/') {
                            buffer.push(' ');
                        }
                        Self::render(
                            &buffer,
                            prompt,
                            &Self::tokenize_buffer(&buffer),
                            &mut stdout,
                        )?;
                        continue;
                    }

                    let lcp = string::lcp(&suggestions);
                    if lcp.len() > partial.len() {
                        let suffix = &lcp[partial.len()..];
                        buffer.push_str(suffix);
                        Self::render(
                            &buffer,
                            prompt,
                            &Self::tokenize_buffer(&buffer),
                            &mut stdout,
                        )?;
                        continue;
                    }

                    tab_cnt += 1;
                    if tab_cnt == 1 {
                        print!("\x07");
                        stdout.flush()?;
                        continue;
                    }

                    if tab_cnt == 2 {
                        tab_cnt = 0;
                        suggestions.sort();
                        print!("\r\n");
                        for suggestion in suggestions {
                            print!("{suggestion}  ");
                        }
                        print!("\r\n");
                        Self::render(
                            &buffer,
                            prompt,
                            &lexed_tokens,
                            &mut stdout,
                        )?;
                        continue;
                    }
                }

                KeyCode::Enter => {
                    print!("\r\n");
                    stdout.flush()?;
                    return Self::submit(&mut stdout, &buffer, &mut ctx);
                }

                _ => {}
            }
        }
    }

    fn submit(
        stdout: &mut io::Stdout,
        buffer: &str,
        input_ctx: &mut InputCtx,
    ) -> Result<ParsedShell, io::Error> {
        print!("\r\n");
        stdout.flush()?;
        input_ctx.history.entries.push(buffer.to_string());
        return Self::parse_buffer(true, input_ctx, &Self::tokenize_buffer(buffer));
    }
}
