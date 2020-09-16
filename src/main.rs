mod error;
mod parser;
mod span;
mod token;

use error::ParseError;
use error::Result;
use parser::{Parse, Parser};
use token::Token;

fn main() {
    println!("Hello, world!");
}

struct Number(u64);

impl Parse for Number {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        let span = parser.try_bump(Token::Number)?;
        let text = parser.text(span);
        Ok(Number(text.parse()?))
    }
}

enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

struct BinOp {
    lhs: Number,
    op: Op,
    rhs: Number,
}

impl Parse for BinOp {
    fn parse(parser: &mut Parser) -> Result<Self> {
        let lhs = parser.parse()?;

        let (token, span) = parser.bump()?;
        let op = match token {
            Token::Plus => Op::Add,
            Token::Minus => Op::Sub,
            Token::Star => Op::Mul,
            Token::Slash => Op::Div,
            _ => {
                return Err(ParseError::UnexpectedToken {
                    span,
                    expected: Token::Plus,
                    got: token,
                })
            }
        };

        let rhs = parser.parse()?;

        Ok(Self { lhs, op, rhs })
    }
}
