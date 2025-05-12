use super::term::Term;
use super::token::Token;

pub fn parse(s: &str) -> Term {
    let lexer = <Token as logos::Logos>::lexer(s);
    read_term(&mut lexer.peekable())
}

// unary   = "false" | "true" | "integer" | "ident"
// binary  = unary | unary "plus" binary
// ternary = binary | binary "question" ternary "colon" ternary
// const   = "const" "ident" "equals" ternary "semicolon" term
// seq     = ternary "semicolon" term
// term    = const | seq | ternary "semicolon"

// TODO: support f(1) and (x: number) => x

fn read_term(iter: &mut std::iter::Peekable<logos::Lexer<'_, Token>>) -> Term {
    let token = iter.peek();
    match token {
        None => unreachable!(),
        Some(token) => {
            let token = token.as_ref().unwrap();
            match token {
                Token::False | Token::True | Token::Integer(_) | Token::Ident(_) => {
                    let t = read_ternary(iter);
                    match iter.peek() {
                        None => return t,
                        Some(token) => {
                            let token = token.as_ref().unwrap();
                            match token {
                                Token::Semicolon => {
                                    assert!(matches!(iter.next(), Some(Ok(Token::Semicolon))));
                                    match iter.peek() {
                                        None => return t,
                                        Some(_) => {
                                            let rest = read_term(iter);
                                            Term::Seq {
                                                body: Box::new(t),
                                                rest: Box::new(rest),
                                            }
                                        }
                                    }
                                }
                                _ => unreachable!(),
                            }
                        }
                    }
                }
                Token::Const => {
                    assert!(matches!(iter.next(), Some(Ok(Token::Const))));
                    let name = match iter.next() {
                        Some(Ok(Token::Ident(name))) => name,
                        _ => unreachable!(),
                    };
                    assert!(matches!(iter.next(), Some(Ok(Token::Equals))));
                    let init = read_ternary(iter);
                    assert!(matches!(iter.next(), Some(Ok(Token::Semicolon))));
                    let rest = read_term(iter);
                    Term::Const {
                        name,
                        init: Box::new(init),
                        rest: Box::new(rest),
                    }
                }
                Token::Plus
                | Token::Quest
                | Token::Colon
                | Token::Semicolon
                | Token::Equals
                | Token::ParenL
                | Token::ParenR
                | Token::Arrow => {
                    unreachable!()
                }
            }
        }
    }
}

fn read_binary(iter: &mut std::iter::Peekable<logos::Lexer<'_, Token>>) -> Term {
    let token = iter.next().unwrap().unwrap();
    let unary = match token {
        Token::False => Term::False,
        Token::True => Term::True,
        Token::Integer(i) => Term::Integer(i),
        Token::Ident(name) => Term::Var { name },
        Token::Plus
        | Token::Quest
        | Token::Colon
        | Token::Semicolon
        | Token::Const
        | Token::Equals
        | Token::ParenL
        | Token::ParenR
        | Token::Arrow => unreachable!(),
    };
    match iter.peek() {
        None => unary,
        Some(token) => {
            let token = token.as_ref().unwrap();
            match token {
                Token::False
                | Token::True
                | Token::Integer(_)
                | Token::Ident(_)
                | Token::Const
                | Token::Equals
                | Token::ParenL
                | Token::ParenR
                | Token::Arrow => unreachable!(),
                Token::Quest | Token::Colon | Token::Semicolon => unary,
                Token::Plus => {
                    assert!(matches!(iter.next(), Some(Ok(Token::Plus))));
                    let left = unary;
                    let right = read_binary(iter);
                    Term::Add {
                        left: Box::new(left),
                        right: Box::new(right),
                    }
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
                Token::Quest => {
                    assert!(matches!(iter.next(), Some(Ok(Token::Quest))));
                    let cond = binary;
                    let thn = read_ternary(iter);
                    assert!(matches!(iter.next(), Some(Ok(Token::Colon))));
                    let els = read_ternary(iter);
                    Term::If {
                        cond: Box::new(cond),
                        thn: Box::new(thn),
                        els: Box::new(els),
                    }
                }
                Token::Colon | Token::Semicolon => binary,
                Token::Ident(_)
                | Token::Const
                | Token::Equals
                | Token::ParenL
                | Token::ParenR
                | Token::Arrow => unreachable!(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_parse(s: &str, expected: Term) {
        let term = parse(s);
        assert_eq!(term, expected);
    }

    #[test]
    fn test_unary() {
        test_parse("true", Term::True);
        test_parse("false", Term::False);
        test_parse("0", Term::Integer(0));
        test_parse(
            "x",
            Term::Var {
                name: "x".to_owned(),
            },
        );
    }

    #[test]
    fn test_binary() {
        test_parse(
            "1 + 2",
            Term::Add {
                left: Box::new(Term::Integer(1)),
                right: Box::new(Term::Integer(2)),
            },
        );
        test_parse(
            "3 + 4 + 5",
            Term::Add {
                left: Box::new(Term::Integer(3)),
                right: Box::new(Term::Add {
                    left: Box::new(Term::Integer(4)),
                    right: Box::new(Term::Integer(5)),
                }),
            },
        );
    }

    #[test]
    fn test_ternary() {
        test_parse(
            "true ? 6 : 7",
            Term::If {
                cond: Box::new(Term::True),
                thn: Box::new(Term::Integer(6)),
                els: Box::new(Term::Integer(7)),
            },
        );
        test_parse(
            "true ? true ? 8 : 9 : 10",
            Term::If {
                cond: Box::new(Term::True),
                thn: Box::new(Term::If {
                    cond: Box::new(Term::True),
                    thn: Box::new(Term::Integer(8)),
                    els: Box::new(Term::Integer(9)),
                }),
                els: Box::new(Term::Integer(10)),
            },
        );
        test_parse(
            "true ? 11 : true ? 12 : 13",
            Term::If {
                cond: Box::new(Term::True),
                thn: Box::new(Term::Integer(11)),
                els: Box::new(Term::If {
                    cond: Box::new(Term::True),
                    thn: Box::new(Term::Integer(12)),
                    els: Box::new(Term::Integer(13)),
                }),
            },
        );
    }

    #[test]
    fn test_const() {
        test_parse(
            "const y = 1; 2",
            Term::Const {
                name: "y".to_owned(),
                init: Box::new(Term::Integer(1)),
                rest: Box::new(Term::Integer(2)),
            },
        );
        test_parse(
            "const z = 3; 4;",
            Term::Const {
                name: "z".to_owned(),
                init: Box::new(Term::Integer(3)),
                rest: Box::new(Term::Integer(4)),
            },
        );
        test_parse(
            "const aa = 5 + 6; aa;",
            Term::Const {
                name: "aa".to_owned(),
                init: Box::new(Term::Add {
                    left: Box::new(Term::Integer(5)),
                    right: Box::new(Term::Integer(6)),
                }),
                rest: Box::new(Term::Var {
                    name: "aa".to_owned(),
                }),
            },
        );
        test_parse(
            "const ab = true ? 7 : 8; ab;",
            Term::Const {
                name: "ab".to_owned(),
                init: Box::new(Term::If {
                    cond: Box::new(Term::True),
                    thn: Box::new(Term::Integer(7)),
                    els: Box::new(Term::Integer(8)),
                }),
                rest: Box::new(Term::Var {
                    name: "ab".to_owned(),
                }),
            },
        );
    }

    #[test]
    fn test_seq() {
        test_parse(
            "1; 2;",
            Term::Seq {
                body: Box::new(Term::Integer(1)),
                rest: Box::new(Term::Integer(2)),
            },
        );
        test_parse(
            "3; 4; 5;",
            Term::Seq {
                body: Box::new(Term::Integer(3)),
                rest: Box::new(Term::Seq {
                    body: Box::new(Term::Integer(4)),
                    rest: Box::new(Term::Integer(5)),
                }),
            },
        );
        test_parse(
            "6; 7",
            Term::Seq {
                body: Box::new(Term::Integer(6)),
                rest: Box::new(Term::Integer(7)),
            },
        );
    }

    #[test]
    fn test_term() {
        test_parse(
            "const x = 1; x; 2",
            Term::Const {
                name: "x".to_owned(),
                init: Box::new(Term::Integer(1)),
                rest: Box::new(Term::Seq {
                    body: Box::new(Term::Var {
                        name: "x".to_owned(),
                    }),
                    rest: Box::new(Term::Integer(2)),
                }),
            },
        );
    }
}
