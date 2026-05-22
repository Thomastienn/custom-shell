use std::io::{self, Write, ErrorKind};
use std::collections::VecDeque;

use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use crossterm::terminal;

use crate::structures::trie::Trie;
use crate::structures::string;

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

pub struct Input<'a> {
    cmd_pref: &'a Trie
}

impl Input<'_> {
    pub fn new(commands: &Trie) -> Input<'_> {
        Input { cmd_pref: commands }
    }

    pub fn read_line(&self, prompt: &str) -> io::Result<String> {
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
                                return Ok(buffer);
                            }
                            _ => {
                                continue
                            }
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
                    let mut suggestions = self.cmd_pref.autocomplete(&buffer);

                    if suggestions.is_empty() {
                        print!("\x07");
                        stdout.flush()?;
                        continue;
                    }
                    if suggestions.len() == 1 {
                        let suffix = &suggestions[0][buffer.len()..];
                        buffer.push_str(suffix);
                        buffer.push(' ');
                        print!("{suffix} ");
                        stdout.flush()?;
                        continue;
                    }

                    let lcp = string::lcp(&suggestions);
                    if lcp.len() > buffer.len() {
                        let suffix = &lcp[buffer.len()..];
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
                    return Ok(buffer);
                }

                _ => {}
            }
        }
    }
}
