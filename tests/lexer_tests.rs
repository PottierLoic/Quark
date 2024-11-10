use quark::lexer::{tokenize, Token};
use std::fs;

#[test]
fn test_tokenize() {
    let code = fs::read_to_string("tests/data/lexer_test_1.quark").expect("Failed to read test file");

    let expected_tokens = vec![
        Token::Let,
        Token::Identifier("x".to_string()),
        Token::Operator("=".to_string()),
        Token::Number(10.0),
        Token::While,
        Token::Identifier("x".to_string()),
        Token::Operator(">".to_string()),
        Token::Number(0.0),
        Token::Arrow,
        Token::Identifier("print".to_string()),
        Token::Identifier("x".to_string()),
        Token::Identifier("x".to_string()),
        Token::Operator("-=".to_string()),
        Token::Number(1.0),
        Token::Semicolon,
        Token::Return,
        Token::Number(0.0),
        Token::Semicolon,
        Token::Eof,
    ];

    let tokens = tokenize(&code).expect("Lexer failed");

    assert_eq!(tokens, expected_tokens);
}