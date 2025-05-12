#[derive(Debug, PartialEq)]
pub enum Type {
    Boolean,
    Integer,
    Func {
        params: Vec<Param>,
        ret_type: Box<Type>,
    },
}

#[derive(Debug, PartialEq)]
pub struct Param {
    name: String,
    typ: Type,
}
