
#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Word(String),
    Number(f64),
    String(String),
    List(Vec<Token>),
}

impl Token {
    pub fn as_string(&self) -> Option<&str> {
        match self {
            Token::Word(word) => Some(word),
            Token::String(string) => Some(string),
            _ => None,
        }
    }

    pub fn as_number(&self) -> Option<f64> {
        match self {
            Token::Number(num) => Some(*num),
            _ => None,
        }
    }

    pub fn as_word(&self) -> Option<&str> {
        match self {
            Token::Word(word) => Some(word),
            _ => None,
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            Token::Word(_) => "word",
            Token::Number(_) => "number",
            Token::String(_) => "string",
            Token::List(_) => "list",
        }
    }
}

pub fn tokenize(source: &str) -> anyhow::Result<Vec<Token>> {
    let mut i = 0;
    let mut tokens = Vec::new();
    let source: Vec<char> = source.chars().collect();

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
        } else if c == '[' {
            i += 1;
            
            tokens.push(Token::List(parse_list(&source, &mut i)?));
        } else {
            current_token.push(c);
        }
        i += 1;
    }

    when_whitespace(&mut tokens, &mut current_token);
    Ok(tokens)
}

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

fn parse_list(source: &Vec<char>, i: &mut usize) -> anyhow::Result<Vec<Token>> {
    let mut list = Vec::new();
    while *i < source.len() && source[*i] != ']' {
        let c = source[*i];
        if c.is_whitespace() {
            when_whitespace(&mut list, &mut String::new());
        } else if c == '"' {
            list.push(Token::String(parse_string(source, i)));
        } else if c == '[' {
            *i += 1;
            list.push(Token::List(parse_list(source, i)?));
        } else {
            let mut current_token = String::new();
            while *i < source.len() && source[*i] != ']' && !source[*i].is_whitespace() {
                current_token.push(source[*i]);
                *i += 1;
            }
            when_whitespace(&mut list, &mut current_token);
        }
        *i += 1;
    }
    Ok(list)
}

fn parse_string(source: &Vec<char>, i: &mut usize) -> String {
    let mut string = String::new();
    *i += 1;
    while *i < source.len() && source[*i] != '"' {
        string.push(source[*i]);
        *i += 1;
    }
    string
}

pub fn untokenize(tokens: &[Token]) -> String {
    tokens
        .iter()
        .map(|token| match token {
            Token::Word(word) => word.clone(),
            Token::Number(num) => num.to_string(),
            Token::String(string) => format!("\"{}\"", string),
            Token::List(list) => {
                let inner = untokenize(list);
                format!("[{}]", inner)
            },
        })
        .collect::<Vec<String>>()
        .join(" ")
}