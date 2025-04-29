use crate::tiny_ts::arith::Term;

#[derive(Debug, PartialEq)]
pub enum Type {
    Boolean,
    Integer,
}

pub fn typecheck(t: Term) -> Type {
    match t {
        Term::False | Term::True => Type::Boolean,
        Term::Integer(_) => Type::Integer,
        Term::Add(left, right) => {
            assert_eq!(typecheck(*left), Type::Integer, "integer expected");
            assert_eq!(typecheck(*right), Type::Integer, "integer expected");
            Type::Integer
        }
        Term::If(cond, thn, els) => {
            assert_eq!(typecheck(*cond), Type::Boolean, "boolean expected");
            let thn_type = typecheck(*thn);
            assert_eq!(
                thn_type,
                typecheck(*els),
                "then and else have different types"
            );
            thn_type
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::tiny_ts::arith::parse;

    #[test]
    fn test() {
        let term = parse("false");
        assert_eq!(typecheck(term), Type::Boolean);

        let term = parse("true");
        assert_eq!(typecheck(term), Type::Boolean);

        let term = parse("0");
        assert_eq!(typecheck(term), Type::Integer);

        let term = parse("true ? 0 : 1 + 2");
        assert_eq!(typecheck(term), Type::Integer);
    }
}
