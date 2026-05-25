#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RedirectOp {
    Read,   // <
    Write,  // >
    Append, // >>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Word(String),
    Redirect {
        fd: Option<u8>,
        op: RedirectOp,
    },
    Background,
    Pipe,
}

pub struct Tokenizer {
    input: String,
    position: usize,
}

impl Tokenizer {
    pub fn new(input: String) -> Self {
        Self { input, position: 0 }
    }

    fn peek(&self) -> Option<char> {
        self.input[self.position..].chars().next()
    }

    fn starts_with(&self, s: &str) -> bool {
        self.input[self.position..].starts_with(s)
    }

    fn advance_char(&mut self, c: char) {
        self.position += c.len_utf8();
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if !c.is_whitespace() {
                break;
            }

            self.advance_char(c);
        }
    }

    fn try_redirect(&mut self) -> Option<Token> {
        let start = self.position;

        let fd = self.try_take_fd();

        let op = if self.starts_with(">>") {
            self.position += 2;
            Some(RedirectOp::Append)
        } else if self.starts_with(">") {
            self.position += 1;
            Some(RedirectOp::Write)
        } else if self.starts_with("<") {
            self.position += 1;
            Some(RedirectOp::Read)
        } else {
            None
        };

        match op {
            Some(op) => Some(Token::Redirect { fd, op }),
            None => {
                self.position = start;
                None
            }
        }
    }

    fn try_take_fd(&mut self) -> Option<u8> {
        let start = self.position;

        let c = self.peek()?;

        if !c.is_ascii_digit() {
            return None;
        }

        self.advance_char(c);

        if self.starts_with(">") || self.starts_with("<") {
            Some(c.to_digit(10).unwrap() as u8)
        } else {
            self.position = start;
            None
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        if self.position >= self.input.len() {
            return None;
        }

        if let Some(redirect) = self.try_redirect() {
            return Some(redirect);
        }

        let mut word = String::new();
        let mut quote: Option<char> = None;
        let mut escape = false;

        while let Some(c) = self.peek() {
            if escape {
                word.push(c);
                self.advance_char(c);
                escape = false;
                continue;
            }

            if c == '\\' && quote != Some('\'') {
                self.advance_char(c);
                escape = true;
                continue;
            }

            if quote.is_none() && (c == '\'' || c == '"') {
                quote = Some(c);
                self.advance_char(c);
                continue;
            }

            if quote == Some(c) {
                quote = None;
                self.advance_char(c);
                continue;
            }

            if quote.is_none() {
                if c.is_whitespace() {
                    break;
                }

                // If it's background operator
                if c == '&' {
                    self.advance_char(c);
                    return Some(Token::Background);
                }

                // Pipe operator
                if c == '|' {
                    self.advance_char(c);
                    return Some(Token::Pipe);
                }

                // If it's redirect
                let start = self.position;
                let is_redirect = self.try_redirect().is_some();
                self.position = start;
                if is_redirect {
                    break;
                }
            }

            word.push(c);
            self.advance_char(c);
        }

        Some(Token::Word(word))
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(token) = self.next_token() {
            tokens.push(token);
        }

        tokens
    }
}
