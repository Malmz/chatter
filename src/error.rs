use crate::{span::Span, token::Token};
use thiserror::Error;

pub type Result<T, E = ParseError> = core::result::Result<T, E>;

#[derive(Debug, Error, Clone)]
pub enum ParseError {
    #[error("unexpected end-of-file")]
    UnexpectedEof { span: Span },

    #[error("unexpected token, expected {expected:?}, got {got:?}")]
    UnexpectedToken {
        span: Span,
        expected: Token,
        got: Token,
    },

    #[error("invalid number literal")]
    InvalidNumber(#[from] std::num::ParseIntError),
}
