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
