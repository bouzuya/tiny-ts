#[derive(Debug, PartialEq)]
pub enum Term {
    False,
    True,
    Integer(u8),
    Add(Box<Term>, Box<Term>),
    If(Box<Term>, Box<Term>, Box<Term>),
}
