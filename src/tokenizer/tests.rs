use crate::{tokenize, Token};

#[test]
fn test_tokenize_empty() {
    let source = "";
    let expect = vec![];
    let tokens = tokenize(source).unwrap();
    assert_tokens(tokens, expect);
}

#[test]
fn test_tokenize_empty_list() {
    let source = "[ ]";
    let expect = vec![Token::List(vec![])];
    let tokens = tokenize(source).unwrap();
    assert_tokens(tokens, expect);
}

#[test]
fn test_tokenize_numbers() {
    let source = "1 2 -3.464348 0099.0 +78.0";
    let expect = vec![Token::Number(1.0), Token::Number(2.0), Token::Number(-3.464348), Token::Number(99.0), Token::Number(78.0)];
    let tokens = tokenize(source).unwrap();
    assert_tokens(tokens, expect);
}

#[test]
fn test_tokenize_one() {
    let source = "hello";
    let expect = vec![Token::Word("hello".to_string())];
    let tokens = tokenize(source).unwrap();
    assert_tokens(tokens, expect);
}

#[test]
fn test_tokenize_string() {
    let source = "\"hello\"";
    let expect = vec![Token::String("hello".to_string())];
    let tokens = tokenize(source).unwrap();
    assert_tokens(tokens, expect);
}

#[test]
fn test_tokenize_mixed() {
    let source = "\"hello\" 1 2\n* 6 /\tOTHER [ ; _ ]";
    let expect = vec![Token::String("hello".to_string()), Token::Number(1.0), Token::Number(2.0), Token::Word("*".to_string()), Token::Number(6.0), Token::Word("/".to_string()), Token::Word("OTHER".to_string()), Token::List(vec![Token::Word(";".to_string()), Token::Word("_".to_string())])];
    let tokens = tokenize(source).unwrap();
    assert_tokens(tokens, expect);
}

#[test]
fn test_tokenize_arithmetics() {
    let source = "1 2 + 6 - 10 * 2 / 2 % 2";
    let expect = vec![Token::Number(1.0), Token::Number(2.0), Token::Word("+".to_string()), Token::Number(6.0), Token::Word("-".to_string()), Token::Number(10.0), Token::Word("*".to_string()), Token::Number(2.0), Token::Word("/".to_string()), Token::Number(2.0), Token::Word("%".to_string()), Token::Number(2.0)];
    let tokens = tokenize(source).unwrap();
    assert_tokens(tokens, expect);
}

#[test]
fn test_tokenize_list() {
    let source = "[ 1 2 + ]";
    let expect = vec![Token::List(vec![Token::Number(1.0), Token::Number(2.0), Token::Word("+".to_string())])];
    let tokens = tokenize(source).unwrap();
    assert_tokens(tokens, expect);
}

fn assert_tokens(tokens: Vec<Token>, expect: Vec<Token>) {
    assert_eq!(tokens.len(), expect.len());
    for (token, expect) in tokens.iter().zip(expect.iter()) {
        assert_eq!(token, expect);
    }
}