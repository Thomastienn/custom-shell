use std::io::{self, Write, ErrorKind};

use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use crossterm::terminal;

use crate::structures::trie::Trie;

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

        loop {
            let Event::Key(key) = event::read()? else {
                continue;
            };

            let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);

            match key.code {
                KeyCode::Char(c) => {
                    if ctrl {
                        match c {
                            'c' => {
                                print!("\r\n");
                                return Err(io::Error::new(ErrorKind::Interrupted, "Interrupted"));
                            }
                            'j' => {
                                print!("\r\n");
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
                    let suggestions = self.cmd_pref.autocomplete(&buffer);

                    if suggestions.is_empty() {
                        print!("\x07");
                        continue;
                    }
                    if suggestions.len() == 1 {
                        let suffix = &suggestions[0][buffer.len()..];
                        buffer.push_str(suffix);
                        print!("{suffix} ");
                        stdout.flush()?;
                        continue;
                    }
                }

                KeyCode::Enter => {
                    print!("\r\n");
                    return Ok(buffer);
                }

                _ => {}
            }
        }
    }
}
