use crate::types::{self, Integer, Product, Sum, Function, Power, Variable, Rational};
use crate::tokens::Token;
use crate::lexer::{Lexer, LexError};
use crate::expression::Expression;
use core::fmt;
use std::vec::IntoIter;
use std::error::Error;

#[derive(Debug)]
pub struct ParseError(String);

impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct Parser {
    tokens: IntoIter<Token>,
    current_token: Option<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        let mut tokens = tokens.into_iter(); 
        Parser {
            current_token: tokens.next(),
            tokens,
        }
    }

    fn advance(&mut self) {
        self.current_token = self.tokens.next();
    }

    pub fn parse(&mut self) -> Result<Expression, ParseError> {
        match self.current_token {
            Some(_) => {
                let result = self.addition()?;
                
                match self.current_token {
                    Some(_) => Err(ParseError("Invalid syntax".to_string())),
                    None => Ok(result),
                }
            }
            None => Err(ParseError("Empty expression".to_string())),
        }
    }

    fn addition(&mut self) -> Result<Expression, ParseError> {
        let mut values = vec![self.multiplication()?];

        while let Some(token) = self.current_token.as_ref() {
            match token {
                Token::Plus => {
                    self.advance();
                    values.push(self.multiplication()?);
                }
                Token::Minus => {
                    self.advance();
                    values.push(neg!(self.multiplication()?));
                }
                _ => break,
            }
        }

        Ok(Sum::new(values).into())
    }

    fn multiplication(&mut self) -> Result<Expression, ParseError> {
        let mut values = vec![self.exponentiation()?];

        while let Some(token) = &self.current_token {
            match token {
                Token::Star => {
                    self.advance();
                    values.push(self.exponentiation()?)
                } 
                Token::Slash => {
                    self.advance();
                    values.push(inv!(self.exponentiation()?));
                }
                _ => break,
            }
        }

        Ok(Product::new(values).into())
    }

    fn exponentiation(&mut self) -> Result<Expression, ParseError> {
        let mut result = self.unary()?;

        while let Some(Token::Caret) = &self.current_token {
            self.advance();

            result = match result {
                Expression::Power(p) => pow!(*p.base, pow!(*p.exp, self.unary()?)),
                _ => pow!(result, self.unary()?)
            }
        }

        Ok(result)
    }

    fn unary(&mut self) -> Result<Expression, ParseError> {
        match self.current_token.clone() {
            Some(Token::Minus) => {
                self.advance();
                Ok(neg!(self.basic()?))
            }
            _ => self.basic(),
        }
    }

    fn basic(&mut self) -> Result<Expression, ParseError> {
        match self.current_token.clone() {
            Some(Token::Integer(x)) => {
                self.advance();
                Ok(int!(x))
            }
            Some(Token::Decimal(x)) => {
                self.advance();
                Ok(Rational::from(x).into())
            }
            Some(Token::Identifier(s)) => {
                self.advance();
                match self.basic() {
                    Ok(u) => Ok(func!(s; u)),
                    Err(..) => Ok(var!(s)),
                }
            }
            Some(Token::Pipe) => {
                self.advance();
                let result = self.addition()?;

                if let Some(Token::Pipe) = &self.current_token {
                    self.advance();
                }

                Ok(func!("abs"; result))
            }
            Some(Token::LeftParen) => {
                self.advance();
                let result = self.addition()?;
                
                if let Some(Token::RightParen) = &self.current_token {
                    self.advance();
                }

                Ok(result)
            }
            Some(Token::LeftBrack) => {
                self.advance();
                let result = self.addition()?;
                
                if let Some(Token::RightBrack) = &self.current_token {
                    self.advance();
                }

                Ok(result)
            }
            _ => Err(ParseError("Invalid syntax".to_string())),
        }
    }
} 
