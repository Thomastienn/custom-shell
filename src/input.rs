use std::collections::VecDeque;
use std::io::{self, ErrorKind, Write};

use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use crossterm::terminal;

use crate::parser::{self, ParsedCommand};
use crate::structures::string;
use crate::structures::trie::Trie;
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
    pub cmd_pref: &'a Trie,
    pub filesystem_pref: &'a Trie,
}

pub struct Input;

impl Input {
    fn parse_buffer(buffer: &str, strict: bool) -> Result<ParsedCommand, io::Error> {
        let mut tokenizer = Tokenizer::new(buffer.to_string());
        let tokens = tokenizer.tokenize();

        parser::parse(tokens, strict)
            .map_err(|e| io::Error::new(ErrorKind::InvalidInput, e))
    }

    pub fn read_line(prompt: &str, ctx: InputCtx) -> Result<ParsedCommand, io::Error> {
        let _raw_mode = RawModeGuard::new()?;

        let mut stdout = io::stdout();
        let mut buffer = String::new();

        print!("{prompt}");
        stdout.flush()?;

        let mut key_presses: VecDeque<KeyCode> = VecDeque::new();
        const MAX_HISTORY_KEYPRESSES: usize = 3;

        let mut tab_cnt = 0;

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
                                print!("\r\n");
                                stdout.flush()?;
                                return Self::parse_buffer(&buffer, true);
                            }
                            _ => continue,
                        }
                    }
                    buffer.push(c);
                    print!("{c}");
                    stdout.flush()?;
                }

                KeyCode::Backspace => {
                    if buffer.pop().is_some() {
                        print!("\x08 \x08");
                        stdout.flush()?;
                    }
                }

                KeyCode::Tab => {
                    let mut suggestions: Vec<String>;
                    let mut partial: &str = &buffer;
                    let partial_word = parsed_cmd.args.len() >= 1;
                    let not_type = buffer.ends_with(" ");
                    if partial_word || not_type {
                        let mut last_token = buffer.split_whitespace().last().unwrap_or("");
                        if not_type {
                            last_token = "";
                        }
                        suggestions = ctx.filesystem_pref.autocomplete(last_token);
                        partial = last_token;
                    } else {
                        suggestions = ctx.cmd_pref.autocomplete(&buffer);
                    }
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
                    return Self::parse_buffer(&buffer, true);
                }

                _ => {}
            }
        }
    }
}
