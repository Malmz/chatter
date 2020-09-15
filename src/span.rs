pub type Spanned<T> = (T, Span);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn from_range(range: logos::Span) -> Self {
        Self {
            start: range.start,
            end: range.end,
        }
    }

    pub fn to_range(&self) -> logos::Span {
        self.start..self.end
    }

    pub fn at(pos: usize) -> Self {
        Span {
            start: pos,
            end: pos,
        }
    }
}
