#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RedirectOp {
    Read,   // <
    Write,  // >
    Append, // >>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WordPart {
    Literal(String),
    Variable(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Word {
    pub parts: Vec<WordPart>
}

impl Word {
    pub fn new() -> Self {
        Self { parts: Vec::new() }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Str(Word),
    Redirect {
        fd: Option<u8>,
        op: RedirectOp,
    },
    Background,
    Pipe,
}

pub struct LexedToken {
    pub token: Token,
    pub start: usize,
    pub end: usize,
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

    pub fn next_token(&mut self) -> Option<LexedToken> {
        let start = self.position;
        self.skip_whitespace();

        if self.position >= self.input.len() {
            return None;
        }

        if let Some(redirect) = self.try_redirect() {
            return Some(
                LexedToken {
                    token: redirect,
                    start,
                    end: self.position,
                }
            );
        }

        let mut quote: Option<char> = None;
        let mut escape = false;
        let mut word = Word::new();
        let mut word_part = String::new();

        while let Some(c) = self.peek() {
            if escape {
                word_part.push(c);
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
                    return Some(
                        LexedToken {
                            token: Token::Background,
                            start,
                            end: self.position,
                        }
                    );
                }

                // Pipe operator
                if c == '|' {
                    self.advance_char(c);
                    return Some(
                        LexedToken {
                            token: Token::Pipe,
                            start,
                            end: self.position,
                        }
                    );
                }

                if c == '$' {
                    word.parts.push(WordPart::Literal(word_part));
                    word_part = String::new();
                    self.advance_char(c);
                    let mut var_name = String::new();
                    if let Some(next) = self.peek() {
                        if next == '{' {
                            self.advance_char(next);
                            while let Some(c) = self.peek() {
                                if c == '}' {
                                    self.advance_char(c);
                                    break;
                                }
                                var_name.push(c);
                                self.advance_char(c);
                            }
                        } else {
                            while let Some(c) = self.peek() {
                                if c.is_alphanumeric() || c == '_' {
                                    var_name.push(c);
                                    self.advance_char(c);
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                    word.parts.push(WordPart::Variable(var_name));
                    continue;
                }

                // If it's redirect
                let start = self.position;
                let is_redirect = self.try_redirect().is_some();
                self.position = start;
                if is_redirect {
                    break;
                }
            }

            word_part.push(c);
            self.advance_char(c);
        }

        word.parts.push(WordPart::Literal(word_part));
        Some(LexedToken {
            token: Token::Str(word),
            start,
            end: self.position,
        })
    }

    pub fn tokenize(&mut self) -> Vec<LexedToken> {
        let mut tokens = Vec::new();

        while let Some(token) = self.next_token() {
            tokens.push(token);
        }

        tokens
    }
}
