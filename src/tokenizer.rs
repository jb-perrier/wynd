#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Word(String),
    Number(f64),
    String(String),
}

pub fn tokenize(source: &str) -> Vec<Token> {
    let mut i = 0;
    let mut tokens = Vec::new();
    let source: Vec<char> = source.chars().collect();

    fn when_whitespace(tokens: &mut Vec<Token>, current_token: &mut String) {
        if !current_token.is_empty() {
            if let Ok(num) = current_token.parse::<f64>() {
                tokens.push(Token::Number(num));
            } else {
                tokens.push(Token::Word(current_token.clone()));
            }
            current_token.clear();
        }
    }

    let mut current_token = String::new();
    while i < source.len() {
        let c = source[i];
        if c.is_whitespace() {
            when_whitespace(&mut tokens, &mut current_token);
        } else if c == '"' {
            i += 1;
            while i < source.len() && source[i] != '"' {
                current_token.push(source[i]);
                i += 1;
            }
            tokens.push(Token::String(current_token.clone()));
            current_token.clear();
        } else {
            current_token.push(c);
        }
        i += 1;
    }

    when_whitespace(&mut tokens, &mut current_token);
    tokens
}

pub fn untokenize(tokens: &[Token]) -> String {
    tokens
        .iter()
        .map(|token| match token {
            Token::Word(word) => word.clone(),
            Token::Number(num) => num.to_string(),
            Token::String(string) => format!("\"{}\"", string),
        })
        .collect::<Vec<String>>()
        .join(" ")
}