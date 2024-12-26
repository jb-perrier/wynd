use crate::{Runtime, RuntimeError, Value, ValueType, WordBuilder, WordCode, Words};

pub fn insert_string(words: &mut Words) {
    words.insert(WordBuilder::new("to-string").code(WordCode::Native(to_string)).description("Convert the top value on the stack to a string").build());
    words.insert(WordBuilder::new("string").code(WordCode::Native(string_type)).description("Put the type id of String on top the stack").build());
}

pub fn string_type(run: &mut Runtime) -> anyhow::Result<()> {
    run.stack.push(Value::Number(ValueType::String.id() as f64));
    Ok(())
}

pub fn to_string(run: &mut Runtime) -> anyhow::Result<()> {
    let value = run.stack.pop().ok_or(RuntimeError::StackUnderflow())?;
    run.stack.push(Value::String(value.to_string()));
    Ok(())
}