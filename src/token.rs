use logos::Logos;

#[derive(Logos, Debug, PartialEq, Copy, Clone)]
pub enum Token {
    #[regex(r"[0-9]+")]
    Number,

    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}
