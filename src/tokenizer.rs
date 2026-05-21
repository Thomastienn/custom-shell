#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Word(String),
    RedirectStdout(String),
    RedirectStderr(String),
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

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        if self.position >= self.input.len() {
            return None;
        }

        if self.starts_with("2>") {
            self.position += 2;
            return Some(Token::RedirectStderr("2>".to_string()));
        }

        if self.starts_with("1>") {
            self.position += 2;
            return Some(Token::RedirectStdout("1>".to_string()));
        }

        if self.starts_with(">") {
            self.position += 1;
            return Some(Token::RedirectStdout(">".to_string()));
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

                if c == '>' {
                    break;
                }

                if self.starts_with("1>") || self.starts_with("2>") {
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
