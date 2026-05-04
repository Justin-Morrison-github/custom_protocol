use std::vec;

use crate::lexer::BCPToken;

type ParseResult<T> = Result<T, String>;

pub struct Parser {
    tokens: Vec<BCPToken>,
    cursor: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum Sign {
    Negative,
    Positive,
}

impl Parser {
    pub fn new(tokens: Vec<BCPToken>) -> Self {
        Self { tokens, cursor: 0 }
    }

    fn parse_field(&mut self) -> ParseResult<BCPField> {
        self.expect(&BCPToken::Field)?;

        let name = self.expect_ident()?;

        self.expect(&BCPToken::Colon)?;

        let index = self.expect_u8()?;

        self.expect(&BCPToken::Pipe)?;

        let len = self.expect_u8()?;

        self.expect(&BCPToken::Pipe)?;

        let sign = self.expect_sign()?;
        let description = self.expect_string()?;

        self.expect(&BCPToken::Semicolon)?;

        Ok(BCPField {
            name,
            index,
            sign,
            len,
            description,
        })
    }

    fn advance(&mut self) -> ParseResult<&BCPToken> {
        let token = self
            .tokens
            .get(self.cursor)
            .ok_or("unexpected end of input")?;
        self.cursor += 1;
        Ok(token)
    }

    fn expect(&mut self, expected: &BCPToken) -> ParseResult<()> {
        let token = self.advance()?;

        if token == expected {
            Ok(())
        } else {
            Err(format!("{:?} != {:?}", token, expected))
        }
    }

    fn expect_u8(&mut self) -> ParseResult<u8> {
        match self.advance()? {
            BCPToken::Number(value) => {
                u8::try_from(*value).map_err(|_| format!("{value} can not be converted to u8"))
            }
            _ => Err("expected number".into()),
        }
    }

    fn expect_u16(&mut self) -> ParseResult<u16> {
        match self.advance()? {
            BCPToken::Number(value) => {
                u16::try_from(*value).map_err(|_| format!("{value} can not be converted to u16"))
            }
            _ => Err("expected number".into()),
        }
    }

    fn expect_sign(&mut self) -> ParseResult<Sign> {
        match self.advance()? {
            BCPToken::Plus => Ok(Sign::Positive),
            BCPToken::Minus => Ok(Sign::Negative),
            _ => Err("expected number".into()),
        }
    }

    fn expect_ident(&mut self) -> ParseResult<String> {
        match self.advance()? {
            BCPToken::Ident(value) => Ok(value.clone()),
            _ => Err("expected Ident".into()),
        }
    }

    fn expect_string(&mut self) -> ParseResult<String> {
        match self.advance()? {
            BCPToken::String(value) => Ok(value.clone()),
            _ => Err("expected string".into()),
        }
    }

    fn check(&self, expected: &BCPToken) -> bool {
        self.tokens.get(self.cursor) == Some(expected)
    }

    fn is_at_end(&self) -> bool {
        let eof = self.check(&BCPToken::Eof);
        eof || self.cursor >= self.tokens.len()
    }

    fn is_at_end_of_message(&self) -> bool {
        self.check(&BCPToken::RightBrace) || self.cursor >= self.tokens.len()
    }

    fn parse_message(&mut self) -> ParseResult<BCPMessage> {
        self.expect(&BCPToken::Message)?;

        let name = self.expect_ident()?;

        // [ID]
        self.expect(&BCPToken::LeftBracket)?;
        let num_bytes: u8 = self.expect_u8()?;
        self.expect(&BCPToken::RightBracket)?;

        let id = self.expect_u16()?;

        let description = self.expect_string()?;

        self.expect(&BCPToken::LeftBrace)?;

        let mut fields: Vec<BCPField> = vec![];

        while !self.is_at_end_of_message() {
            fields.push(self.parse_field()?);
        }

        self.expect(&BCPToken::RightBrace)?;

        Ok(BCPMessage {
            info: BCPMessageInfo {
                id,
                name,
                num_bytes,
                description,
            },
            fields,
        })
    }

    pub fn parse_tokens(&mut self) -> ParseResult<BCPFile> {
        let mut messages: Vec<BCPMessage> = Vec::new();

        while !self.is_at_end() {
            messages.push(self.parse_message()?);
        }

        Ok(BCPFile { messages })
    }
}

#[derive(Debug)]
pub struct BCPFile {
    pub(crate) messages: Vec<BCPMessage>,
}

#[derive(Debug)]
pub struct BCPMessage {
    pub(crate) info: BCPMessageInfo,
    pub(crate) fields: Vec<BCPField>,
}

#[derive(Debug)]
pub struct BCPMessageInfo {
    pub(crate) id: u16,
    pub(crate) name: String,
    pub(crate) num_bytes: u8,
    pub(crate) description: String,
}

#[derive(Debug)]

pub struct BCPField {
    pub(crate) name: String,
    pub(crate) index: u8,
    pub(crate) sign: Sign,
    pub(crate) len: u8,
    pub(crate) description: String,
}
