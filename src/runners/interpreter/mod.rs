mod error;

pub use error::*;

use crate::Token;

use super::{runtime, Runtime, Value};

pub fn interpret(str: &[Token], runtime: &mut impl Runtime) -> Result<(), InterpreterError> {

    
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