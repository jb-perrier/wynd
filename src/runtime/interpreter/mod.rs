use super::{InterpretedWordParameters, Value};
use crate::{Module, StackRefMut, Token};

pub fn interpret<'a>(input: &[Token], values: &'a mut StackRefMut<Value>, modules: &mut [Module]) {
    let mut ip = 0;
    while ip < input.len() {
        let token = &input[ip];
        match token {
            Token::Word(word) => interpret_word(word, values, modules),
            Token::Number(number) => {
                values.push(Value::Number(*number));
            }
            Token::String(string) => {
                values.push(Value::String(string.clone()));
            }
            _ => panic!("Unexpected token: {:?}", token),
        }
        ip += 1;
    }
}

#[inline]
pub fn interpret_word(word: &str, values: &mut StackRefMut<Value>, modules: &mut [Module]) {
    match modules
        .iter()
        .find(|module| module.words().iter().any(|w| w.name() == word))
    {
        Some(module) => {
            let word = module.words().iter().find(|w| w.name() == word).unwrap();
            match word.implementation().unwrap() {
                crate::WordImplementation::Tokens(vec) => interpret(&vec.clone(), values, modules),
                crate::WordImplementation::Native {
                    interpreted,
                    compiled: _,
                } => {
                    if let Some(func) = interpreted {
                        let mut intparams = InterpretedWordParameters::new(values);
                        func(&mut intparams);
                    }
                }
            }
        }
        None => panic!("Word not found: {}", word),
    };
}
