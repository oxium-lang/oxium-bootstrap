#[cfg(test)]
mod tests_lexer {
    use crate::token::Token;
    use crate::lexer::Lexer;

    #[test]
    fn test_if_else() {
        let input = "tests/lexer/test_if_else.oxi";
        let lexer = Lexer::new(&input.to_string());
        let tokens = lexer.lex();
        let expected_tokens = vec![
            Token::Keyword(1, 1, "if".to_string()),
            Token::Seperator(1, 4, '('),
            Token::Identifier(1, 5, "x".to_string()),
            Token::Operator(1, 7, ">".to_string()),
            Token::IntLit(1, 9, "0".to_string()),
            Token::Seperator(1, 10, ')'),
            Token::Seperator(1, 12, '{'),
            Token::Keyword(1, 14, "ret".to_string()),
            Token::Keyword(1, 18, "true".to_string()),
            Token::Seperator(1, 22, ';'),
            Token::Seperator(1, 24, '}'),
            Token::Keyword(1, 26, "else".to_string()),
            Token::Seperator(1, 31, '{'),
            Token::Keyword(1, 33, "ret".to_string()),
            Token::Keyword(1, 37, "false".to_string()),
            Token::Seperator(1, 42, ';'),
            Token::Seperator(1, 44, '}'),
            Token::Eof,
        ];
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_if() {
        let input = "tests/lexer/test_if.oxi";
        let lexer = Lexer::new(&input.to_string());
        let tokens = lexer.lex();
        let expected_tokens = vec![
            Token::Keyword(1, 1, "if".to_string()),
            Token::Seperator(1, 4, '('),
            Token::Identifier(1, 5, "x".to_string()),
            Token::Operator(1, 7, ">".to_string()),
            Token::IntLit(1, 9, "0".to_string()),
            Token::Seperator(1, 10, ')'),
            Token::Seperator(1, 12, '{'),
            Token::Keyword(1, 14, "ret".to_string()),
            Token::IntLit(1, 18, "0".to_string()),
            Token::Seperator(1, 19, ';'),
            Token::Seperator(1, 21, '}'),
            Token::Eof
        ];
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_string() {
        let lexer = Lexer::new(&"tests/lexer/test_string.oxi".to_string());
        let tokens = lexer.lex();

        assert_eq!(tokens.len(), 3);
        assert_eq!(
            tokens[0],
            Token::StringLit(1, 3, r#"\u001b[32;m"#.to_string())
        );
        assert_eq!(tokens[1], Token::StringLit(2,18,String::from("\"x")));
        assert_eq!(tokens[2], Token::Eof);
    }

}