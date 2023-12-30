use crate::tokens::Token;
use core::fmt;
use std::vec::IntoIter;
use std::error::Error;

#[derive(Debug)]
pub struct LexError(String);

impl Error for LexError {}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct Lexer {
    chars: IntoIter<char>,
    current_char: Option<char>
}

impl Lexer {
    pub fn new(text: impl Into<String>) -> Self {
        let mut chars = text
            .into()
            .chars()
            .collect::<Vec<_>>()
            .into_iter();
        Lexer { 
            current_char: chars.next(),
            chars
        }
    }

    pub fn tokens(mut self) -> Result<Vec<Token>, LexError> {
        let mut tokens = Vec::new();

        while let Some(ch) = self.current_char {
            if ch.is_whitespace() {
                self.advance();
                continue;
            }

            let token = match ch {
                'a'..='z'       => self.generate_identifier(),
                '.' | '0'..='9' => self.generate_number()?,
                _               => self.generate_math_token()?,
            };

            tokens.push(token);
        }

        Ok(tokens)
    }

    fn advance(&mut self) {
        self.current_char = self.chars.next()
    }

    fn generate_identifier(&mut self) -> Token {
        let mut identifier = self.current_char.unwrap().to_string();
        self.advance();

        // Keep looping until no more characters are found.
        while let Some(ch @ 'a'..='z') = self.current_char {
            identifier.push(ch);
            self.advance();
        }

        Token::Identifier(identifier)
    }

    fn generate_number(&mut self) -> Result<Token, LexError> {
        let mut number_str = self.current_char.unwrap().to_string();
        self.advance();

        // Keep looping until no more periods or digits are found.
        while let Some(ch @ '.' | ch @ '0'..='9') = self.current_char {
            number_str.push(ch);
            self.advance();
        }

        number_str
            .parse::<i32>()
            .map(Token::Integer)
            .or_else(|_| number_str.parse::<f32>().map(Token::Decimal))
            .map_err(|_| LexError(format!("Invalid number syntax `{}`", number_str)))
    }

    fn generate_math_token(&mut self) -> Result<Token, LexError> {
        let token = match self.current_char.unwrap() {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Star,
            '/' => Token::Slash,
            '^' => Token::Caret,
            '|' => Token::Pipe,
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '[' => Token::LeftBrack,
            ']' => Token::RightBrack,
            // Invalid character.
            ch => return Err(LexError(format!("Invalid character `{}`", ch)))
        };

        self.advance();
        Ok(token)
    }
}
