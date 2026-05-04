#[derive(Debug, Clone, PartialEq)]
pub enum BCPToken {
    Message,
    Field,

    Ident(String),
    Number(u64),
    String(String),

    LeftBracket,       // [
    RightBracket,      // ]
    LeftBrace,         // {
    RightBrace,        // }
    Colon,             // :
    Pipe,              // |
    Plus,              // +
    Minus,             // -
    Semicolon,         // ;
    LeftAngleBracket,  // <
    RightAngleBracket, // >

    Eof,
}

pub struct Lexer {
    chars: Vec<char>,
    cursor: usize,
}

impl Lexer {
    pub fn new(text: &str) -> Self {
        Self {
            chars: text.chars().collect(),
            cursor: 0,
        }
    }

    pub fn decode(&mut self) -> Result<Vec<BCPToken>, String> {
        let mut tokens: Vec<BCPToken> = Vec::new();

        loop {
            let token = self.read_next_token()?;
            let is_eof = token == BCPToken::Eof;

            tokens.push(token);

            if is_eof {
                return Ok(tokens);
            }
        }
    }

    pub fn get_char_at_cursor(&self) -> Option<char> {
        if self.cursor < self.chars.len() {
            let c = self.chars[self.cursor];
            Some(c)
        } else {
            None
        }
    }

    pub fn read_next_token(&mut self) -> Result<BCPToken, String> {
        self.skip_whitespace();

        let Some(ch) = self.advance() else {
            return Ok(BCPToken::Eof);
        };

        match ch {
            '[' => Ok(BCPToken::LeftBracket),
            ']' => Ok(BCPToken::RightBracket),
            '{' => Ok(BCPToken::LeftBrace),
            '}' => Ok(BCPToken::RightBrace),
            '>' => Ok(BCPToken::RightAngleBracket),
            '<' => Ok(BCPToken::LeftAngleBracket),
            ':' => Ok(BCPToken::Colon),
            '|' => Ok(BCPToken::Pipe),
            '+' => Ok(BCPToken::Plus),
            '-' => Ok(BCPToken::Minus),
            ';' => Ok(BCPToken::Semicolon),
            '"' => self.read_string(),
            c if c.is_ascii_digit() => self.read_number(c),
            c if is_ident_start(c) => self.read_ident_or_keyword(c),
            c => Err(format!("Unexpected character: {}", c)),
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.cursor).copied()
    }

    fn read_ident_or_keyword(&mut self, first: char) -> Result<BCPToken, String> {
        let mut value = first.to_string();

        while matches!(self.peek(), Some(c) if is_ident_continue(c)) {
            value.push(self.advance().unwrap());
        }

        match value.as_str() {
            "MESSAGE" => Ok(BCPToken::Message),
            "FIELD" => Ok(BCPToken::Field),
            _ => Ok(BCPToken::Ident(value)),
        }
    }

    fn read_string(&mut self) -> Result<BCPToken, String> {
        let mut value = String::new();
        while let Some(ch) = self.advance() {
            if ch == '"' {
                return Ok(BCPToken::String(value));
            }
            value.push(ch);
        }

        Err("Unterminated string".into())
    }

    fn read_number(&mut self, first: char) -> Result<BCPToken, String> {
        let mut value = first.to_string();

        while matches!(self.peek(), Some(c) if c.is_ascii_digit()) {
            value.push(self.advance().unwrap());
        }

        let number = value
            .parse::<u64>()
            .map_err(|_| format!("Invalid number: {}", value))?;

        Ok(BCPToken::Number(number))
    }

    pub fn skip_whitespace(&mut self) {
        while let Some(y) = self.get_char_at_cursor() {
            if y.is_whitespace() {
                self.cursor += 1;
            } else {
                break;
            }
        }
    }

    pub fn advance(&mut self) -> Option<char> {
        if self.cursor < self.chars.len() {
            let c = self.chars[self.cursor];
            self.cursor += 1;
            Some(c)
        } else {
            None
        }
    }
}

fn is_ident_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_ident_continue(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}
