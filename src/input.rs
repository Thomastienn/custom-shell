use std::collections::VecDeque;
use std::io::{self, ErrorKind, Write};
use std::path::PathBuf;

use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use crossterm::terminal;

use crate::parser::{self, ParsedShell};
use crate::runnable::complete::Complete;
use crate::runnable::{CommandMap, CompletionPath, History};
use crate::structures::string;
use crate::structures::trie::{Trie};
use crate::tokenizer::Tokenizer;

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
    pub history: &'a mut History,
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
    fn parse_buffer(buffer: &str, strict: bool) -> Result<ParsedShell, io::Error> {
        let mut tokenizer = Tokenizer::new(buffer.to_string());
        let tokens = tokenizer.tokenize();

        parser::parse(tokens, strict).map_err(|e| io::Error::new(ErrorKind::InvalidInput, e))
    }

    pub fn read_line(prompt: &str, ctx: InputCtx) -> Result<ParsedShell, io::Error> {
        let _raw_mode = RawModeGuard::new()?;

        let mut stdout = io::stdout();
        let mut buffer = String::new();

        print!("{prompt}");
        stdout.flush()?;

        let mut key_presses: VecDeque<KeyCode> = VecDeque::new();
        const MAX_HISTORY_KEYPRESSES: usize = 3;

        let mut tab_cnt = 0;
        let mut current_history = ctx.history.entries.len(); // 1-indexed since its usize

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

            let parsed_cmd = Self::parse_buffer(&buffer, false).unwrap();

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
                                return Self::submit(&mut stdout, &buffer, ctx.history);
                            }
                            'z' => {
                                dbg!(&parsed_cmd);
                            }
                            _ => continue,
                        }
                    }
                    buffer.push(c);
                    print!("{c}");
                    stdout.flush()?;
                }

                KeyCode::Up => {
                    if current_history > 0 {
                        current_history -= 1;
                        buffer = ctx.history.entries[current_history].clone();
                        print!("\r{prompt}{buffer}\x1b[K");
                        stdout.flush()?;
                    }
                }

                KeyCode::Down => {
                    if current_history + 1 < ctx.history.entries.len() {
                        current_history += 1;
                        buffer = ctx.history.entries[current_history].clone();
                        print!("\r{prompt}{buffer}\x1b[K");
                        stdout.flush()?;
                    }
                }

                KeyCode::Backspace => {
                    if buffer.pop().is_some() {
                        print!("\x08 \x08");
                        stdout.flush()?;
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
                        autocomplete =
                            Some(SuggestionType::Complete(prev, cur_arg, path_str));
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
                                buffer.len()
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
                        print!("{suffix}");
                        if !suggestions[0].ends_with('/') {
                            print!(" ");
                            buffer.push(' ');
                        }
                        stdout.flush()?;
                        continue;
                    }

                    let lcp = string::lcp(&suggestions);
                    if lcp.len() > partial.len() {
                        let suffix = &lcp[partial.len()..];
                        buffer.push_str(suffix);
                        print!("{suffix}");
                        stdout.flush()?;
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
                        print!("\r\n{prompt}{buffer}");
                        stdout.flush()?;
                        continue;
                    }
                }

                KeyCode::Enter => {
                    print!("\r\n");
                    stdout.flush()?;
                    return Self::submit(&mut stdout, &buffer, ctx.history);
                }

                _ => {}
            }
        }
    }

    fn submit(stdout: &mut io::Stdout, buffer: &str, history: &mut History) -> Result<ParsedShell, io::Error> {
        print!("\r\n");
        stdout.flush()?;
        history.entries.push(buffer.to_string());
        return Self::parse_buffer(buffer, true);
    }
}
