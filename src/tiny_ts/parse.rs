use crate::tiny_ts::term::Term;
use crate::tiny_ts::token::Token;

pub fn parse(s: &str) -> Term {
    let lexer = <Token as logos::Logos>::lexer(s);
    read_ternary(&mut lexer.peekable())
}

// unary = false | true | integer
// binary = unary | unary plus binary
// ternary = binary | binary question ternary colon ternary

fn read_binary(iter: &mut std::iter::Peekable<logos::Lexer<'_, Token>>) -> Term {
    let token = iter.next().unwrap().unwrap();
    let unary = match token {
        Token::False => Term::False,
        Token::True => Term::True,
        Token::Integer(i) => Term::Integer(i),
        Token::Plus | Token::Question | Token::Colon => unreachable!(),
    };
    match iter.peek() {
        None => unary,
        Some(token) => {
            let token = token.as_ref().unwrap();
            match token {
                Token::False | Token::True | Token::Integer(_) => unreachable!(),
                Token::Question | Token::Colon => unary,
                Token::Plus => {
                    assert!(matches!(iter.next(), Some(Ok(Token::Plus))));
                    let left = unary;
                    let right = read_binary(iter);
                    Term::Add(Box::new(left), Box::new(right))
                }
            }
        }
    }
}

fn read_ternary(iter: &mut std::iter::Peekable<logos::Lexer<'_, Token>>) -> Term {
    let binary = read_binary(iter);
    match iter.peek() {
        None => binary,
        Some(token) => {
            let token = token.as_ref().unwrap();
            match token {
                Token::False | Token::True | Token::Integer(_) | Token::Plus => {
                    unreachable!()
                }
                Token::Question => {
                    assert!(matches!(iter.next(), Some(Ok(Token::Question))));
                    let cond = binary;
                    let thn = read_ternary(iter);
                    assert!(matches!(iter.next(), Some(Ok(Token::Colon))));
                    let els = read_ternary(iter);
                    Term::If(Box::new(cond), Box::new(thn), Box::new(els))
                }
                Token::Colon => binary,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        fn test_parse(s: &str, expected: Term) {
            let term = parse(s);
            assert_eq!(term, expected);
        }

        test_parse("true", Term::True);
        test_parse("false", Term::False);
        test_parse("0", Term::Integer(0));
        test_parse(
            "1 + 2",
            Term::Add(Box::new(Term::Integer(1)), Box::new(Term::Integer(2))),
        );
        test_parse(
            "3 + 4 + 5",
            Term::Add(
                Box::new(Term::Integer(3)),
                Box::new(Term::Add(
                    Box::new(Term::Integer(4)),
                    Box::new(Term::Integer(5)),
                )),
            ),
        );
        test_parse(
            "true ? 6 : 7",
            Term::If(
                Box::new(Term::True),
                Box::new(Term::Integer(6)),
                Box::new(Term::Integer(7)),
            ),
        );
        test_parse(
            "true ? true ? 8 : 9 : 10",
            Term::If(
                Box::new(Term::True),
                Box::new(Term::If(
                    Box::new(Term::True),
                    Box::new(Term::Integer(8)),
                    Box::new(Term::Integer(9)),
                )),
                Box::new(Term::Integer(10)),
            ),
        );
        test_parse(
            "true ? 11 : true ? 12 : 13",
            Term::If(
                Box::new(Term::True),
                Box::new(Term::Integer(11)),
                Box::new(Term::If(
                    Box::new(Term::True),
                    Box::new(Term::Integer(12)),
                    Box::new(Term::Integer(13)),
                )),
            ),
        );
    }
}
