use crate::{Runtime, RuntimeError, Value, ValueType, WordBuilder, WordCode};

pub fn insert_number(words: &mut crate::Words) {
    words.insert(WordBuilder::new("to-number").code(WordCode::Native(to_number)).description("Convert value to a number").build());
    words.insert(WordBuilder::new("number").code(WordCode::Native(number_type)).description("Put the type id of Number on top the stack").build());
}

pub fn number_type(run: &mut Runtime) -> anyhow::Result<()> {
    run.stack.push(Value::Number(ValueType::Number.id() as f64));
    Ok(())
}

pub fn to_number(run: &mut Runtime) -> anyhow::Result<()> {
    let value = run.stack.pop().ok_or(RuntimeError::StackUnderflow())?;
    match value {
        Value::Number(_) => {
            run.stack.push(value);
        }
        Value::String(string) => {
            let number = string.parse::<f64>()?;
            run.stack.push(Value::Number(number));
        }
        _ => {
            return Err(RuntimeError::CannotConvertValue { from: value.value_type().as_str(), to: "number" }.into());
        }
    }
    Ok(())
}