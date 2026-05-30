use crate::tokenizer::{LexedToken, RedirectOp, Token, WordPart};

enum Color {
    Red,
    _Green,
    _Yellow,
    Blue,
    _Magenta,
    _Cyan,
    White,
}

fn color_to_ansi(color: Color) -> &'static str {
    match color {
        Color::Red => "\x1b[31m",
        Color::_Green => "\x1b[32m",
        Color::_Yellow => "\x1b[33m",
        Color::Blue => "\x1b[34m",
        Color::_Magenta => "\x1b[35m",
        Color::_Cyan => "\x1b[36m",
        Color::White => "\x1b[37m",
    }
}

fn color_str(s: &str, color: Color) -> String {
    format!("{}{}{}", color_to_ansi(color), s, "\x1b[0m")
}

pub fn color_token(raw: &str, lex_token: &LexedToken) -> String {
    match &lex_token.token {
        Token::Str(_) => {
            color_str(raw, Color::White)
        }

        Token::Redirect { .. } => {
            color_str(raw, Color::Red)
        }

        Token::Background => {
            color_str(raw, Color::Blue)
        }

        Token::Pipe => {
            color_str(raw, Color::Blue)
        }
    }
}
