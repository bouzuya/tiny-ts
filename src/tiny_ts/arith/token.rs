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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
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
}
