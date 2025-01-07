mod error;

pub use error::*;

use crate::Token;

use super::{runtime, Runtime, Value};

pub fn interpret(str: &[Token], runtime: &mut impl Runtime) -> Result<(), InterpreterError> {
    let mut i = 0;
    loop {
        let token = &str[i];
        match token {
            Token::Number(n) => runtime.push_value(Value::Number(*n)),
            Token::Word(w) => {
                match w.as_str() {
                    "+" => add(runtime)?,
                    "-" => sub(runtime)?,
                    "*" => mul(runtime)?,
                    "/" => div(runtime)?,
                    _ => return Err(InterpreterError::UnknownWord(w.clone())),
                }
            }
        }
    }
    Ok(())
}

#[inline]
pub fn add(runtime: &mut impl Runtime) -> Result<(), InterpreterError> {
    let a = *runtime.peek_value(0).as_number().unwrap();
    let b = *runtime.peek_value(1).as_number().unwrap();
    let result = a + b;
    runtime.pop_value();
    runtime.pop_value();
    runtime.push_value(Value::Number(result));
    Ok(())
}

