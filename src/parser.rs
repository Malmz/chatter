use crate::error::Result;
use crate::{error::ParseError, span::Span, span::Spanned, token::Token};

pub struct Parser<'a> {
    lexer: logos::Lexer<'a, Token>,
    peek: Option<Spanned<Token>>,
}

impl<'a> Parser<'a> {
    fn next(&mut self) -> Option<Spanned<Token>> {
        self.lexer
            .next()
            .map(|t| (t, Span::from_range(self.lexer.span())))
    }

    pub fn bump(&mut self) -> Result<Spanned<Token>> {
        let next = self.next();
        let res = std::mem::replace(&mut self.peek, next);

        res.ok_or(ParseError::UnexpectedEof {
            span: Span::at(self.lexer.source().len()),
        })
    }

    pub fn peek(&self) -> Result<Spanned<Token>> {
        self.peek.ok_or(ParseError::UnexpectedEof {
            span: Span::at(self.lexer.source().len()),
        })
    }

    pub fn try_bump(&mut self, expect: Token) -> Result<Span> {
        let (got, span) = self.bump()?;
        if got == expect {
            Ok(span)
        } else {
            Err(ParseError::UnexpectedToken {
                span,
                expected: expect,
                got,
            })
        }
    }

    pub fn text(&self, span: Span) -> &str {
        &self.lexer.source()[span.to_range()]
    }
}

pub trait Parse: Sized {
    fn parse(parser: &mut Parser) -> Result<Self>;
}
