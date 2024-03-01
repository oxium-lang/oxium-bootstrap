#[cfg(test)]
mod tests {
    use crate::token::Token;
    use crate::lexer::Lexer;

    #[test]
    fn test_if_else() {
        // Create a sample input string
        let input = "tests/test_if_else.oxi";

        // Create a lexer instance with the sample input string
        let lexer = Lexer::new(&input.to_string());

        // Call the lex method to tokenize the input string
        let tokens = lexer.lex();

        // Define the expected tokens based on the input string
        let expected_tokens = vec![
            Token::Keyword(1, 1, "if".to_string()),
            Token::Seperator(1, 3, '('),
            Token::Identifier(1, 5, "x".to_string()),
            Token::Operator(1, 6, ">".to_string()),
            Token::IntLit(1, 9, "0".to_string()),
            Token::Seperator(1, 11, ')'),
            Token::Seperator(1, 13, '{'),
            Token::Keyword(1, 15, "ret".to_string()),
            Token::Keyword(1, 23, "true".to_string()),
            Token::Seperator(1, 27, ';'),
            Token::Seperator(1, 29, '}'),
            Token::Keyword(1, 31, "else".to_string()),
            Token::Seperator(1, 36, '{'),
            Token::Keyword(1, 38, "return".to_string()),
            Token::Keyword(1, 46, "false".to_string()),
            Token::Seperator(1, 51, ';'),
            Token::Seperator(1, 53, '}'),
            Token::Eof,
        ];

        println!("{:#?}", tokens);

        // Assert that the actual tokens match the expected tokens
        assert_eq!(tokens, expected_tokens);
    }
}
