#[derive(Debug, PartialEq, logos::Logos)]
#[logos(skip r"\s+")]
pub enum Token {
    #[token("false")]
    False,
    #[token("true")]
    True,
    #[regex("0|[1-9][0-9]{0,}", |lexer| lexer.slice().parse::<u8>().ok())]
    Integer(u8),
    #[token("+")]
    Plus,
    #[token("?")]
    Quest,
    #[token(":")]
    Colon,
    #[regex("[a-zA-Z_][a-zA-Z0-9_]{0,}", |lexer| lexer.slice().to_string())]
    Ident(String),
    #[token(";")]
    Semicolon,
    #[token("const")]
    Const,
    #[token("=")]
    Equals,
    #[token("(")]
    ParenL,
    #[token(",")]
    Comma,
    #[token(")")]
    ParenR,
    #[token("=>")]
    Arrow,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arith() {
        let mut lexer = <Token as logos::Logos>::lexer("true ? 0 : 1 + 2");

        assert_eq!(lexer.next(), Some(Ok(Token::True)));
        assert_eq!(lexer.span(), 0..4);
        assert_eq!(lexer.slice(), "true");

        assert_eq!(lexer.next(), Some(Ok(Token::Quest)));
        assert_eq!(lexer.span(), 5..6);
        assert_eq!(lexer.slice(), "?");

        assert_eq!(lexer.next(), Some(Ok(Token::Integer(0))));
        assert_eq!(lexer.span(), 7..8);
        assert_eq!(lexer.slice(), "0");

        assert_eq!(lexer.next(), Some(Ok(Token::Colon)));
        assert_eq!(lexer.span(), 9..10);
        assert_eq!(lexer.slice(), ":");

        assert_eq!(lexer.next(), Some(Ok(Token::Integer(1))));
        assert_eq!(lexer.span(), 11..12);
        assert_eq!(lexer.slice(), "1");

        assert_eq!(lexer.next(), Some(Ok(Token::Plus)));
        assert_eq!(lexer.span(), 13..14);
        assert_eq!(lexer.slice(), "+");

        assert_eq!(lexer.next(), Some(Ok(Token::Integer(2))));
        assert_eq!(lexer.span(), 15..16);
        assert_eq!(lexer.slice(), "2");

        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_basic() {
        let mut lexer = <Token as logos::Logos>::lexer("x");
        assert_eq!(lexer.next(), Some(Ok(Token::Ident("x".to_owned()))));
        assert_eq!(lexer.span(), 0..1);
        assert_eq!(lexer.slice(), "x");
        assert_eq!(lexer.next(), None);

        let mut lexer = <Token as logos::Logos>::lexer("f");
        assert_eq!(lexer.next(), Some(Ok(Token::Ident("f".to_owned()))));
        assert_eq!(lexer.span(), 0..1);
        assert_eq!(lexer.slice(), "f");
        assert_eq!(lexer.next(), None);

        let mut lexer = <Token as logos::Logos>::lexer("(x: number) => x");
        assert_eq!(lexer.next(), Some(Ok(Token::ParenL)));
        assert_eq!(lexer.span(), 0..1);
        assert_eq!(lexer.slice(), "(");
        assert_eq!(lexer.next(), Some(Ok(Token::Ident("x".to_owned()))));
        assert_eq!(lexer.span(), 1..2);
        assert_eq!(lexer.slice(), "x");
        assert_eq!(lexer.next(), Some(Ok(Token::Colon)));
        assert_eq!(lexer.span(), 2..3);
        assert_eq!(lexer.slice(), ":");
        assert_eq!(lexer.next(), Some(Ok(Token::Ident("number".to_owned()))));
        assert_eq!(lexer.span(), 4..10);
        assert_eq!(lexer.slice(), "number");
        assert_eq!(lexer.next(), Some(Ok(Token::ParenR)));
        assert_eq!(lexer.span(), 10..11);
        assert_eq!(lexer.slice(), ")");
        assert_eq!(lexer.next(), Some(Ok(Token::Arrow)));
        assert_eq!(lexer.span(), 12..14);
        assert_eq!(lexer.slice(), "=>");
        assert_eq!(lexer.next(), Some(Ok(Token::Ident("x".to_owned()))));
        assert_eq!(lexer.span(), 15..16);
        assert_eq!(lexer.slice(), "x");
        assert_eq!(lexer.next(), None);

        let mut lexer = <Token as logos::Logos>::lexer("f(1)");
        assert_eq!(lexer.next(), Some(Ok(Token::Ident("f".to_owned()))));
        assert_eq!(lexer.span(), 0..1);
        assert_eq!(lexer.slice(), "f");
        assert_eq!(lexer.next(), Some(Ok(Token::ParenL)));
        assert_eq!(lexer.span(), 1..2);
        assert_eq!(lexer.slice(), "(");
        assert_eq!(lexer.next(), Some(Ok(Token::Integer(1))));
        assert_eq!(lexer.span(), 2..3);
        assert_eq!(lexer.slice(), "1");
        assert_eq!(lexer.next(), Some(Ok(Token::ParenR)));
        assert_eq!(lexer.span(), 3..4);
        assert_eq!(lexer.slice(), ")");
        assert_eq!(lexer.next(), None);

        let mut lexer = <Token as logos::Logos>::lexer("g(2, 3)");
        assert_eq!(lexer.next(), Some(Ok(Token::Ident("g".to_owned()))));
        assert_eq!(lexer.span(), 0..1);
        assert_eq!(lexer.slice(), "g");
        assert_eq!(lexer.next(), Some(Ok(Token::ParenL)));
        assert_eq!(lexer.span(), 1..2);
        assert_eq!(lexer.slice(), "(");
        assert_eq!(lexer.next(), Some(Ok(Token::Integer(2))));
        assert_eq!(lexer.span(), 2..3);
        assert_eq!(lexer.slice(), "2");
        assert_eq!(lexer.next(), Some(Ok(Token::Comma)));
        assert_eq!(lexer.span(), 3..4);
        assert_eq!(lexer.slice(), ",");
        assert_eq!(lexer.next(), Some(Ok(Token::Integer(3))));
        assert_eq!(lexer.span(), 5..6);
        assert_eq!(lexer.slice(), "3");
        assert_eq!(lexer.next(), Some(Ok(Token::ParenR)));
        assert_eq!(lexer.span(), 6..7);
        assert_eq!(lexer.slice(), ")");
        assert_eq!(lexer.next(), None);

        let mut lexer = <Token as logos::Logos>::lexer("const x = 1; x");
        assert_eq!(lexer.next(), Some(Ok(Token::Const)));
        assert_eq!(lexer.span(), 0..5);
        assert_eq!(lexer.slice(), "const");
        assert_eq!(lexer.next(), Some(Ok(Token::Ident("x".to_owned()))));
        assert_eq!(lexer.span(), 6..7);
        assert_eq!(lexer.slice(), "x");
        assert_eq!(lexer.next(), Some(Ok(Token::Equals)));
        assert_eq!(lexer.span(), 8..9);
        assert_eq!(lexer.slice(), "=");
        assert_eq!(lexer.next(), Some(Ok(Token::Integer(1))));
        assert_eq!(lexer.span(), 10..11);
        assert_eq!(lexer.slice(), "1");
        assert_eq!(lexer.next(), Some(Ok(Token::Semicolon)));
        assert_eq!(lexer.span(), 11..12);
        assert_eq!(lexer.slice(), ";");
        assert_eq!(lexer.next(), Some(Ok(Token::Ident("x".to_owned()))));
        assert_eq!(lexer.span(), 13..14);
        assert_eq!(lexer.slice(), "x");
        assert_eq!(lexer.next(), None);
    }
}
