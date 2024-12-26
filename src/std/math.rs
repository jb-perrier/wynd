use crate::{Runtime, RuntimeError, Value, Word};

pub fn insert_math(words: &mut crate::Words) {
    words.insert("+", Word::Native(add));
    words.insert("-", Word::Native(sub));
    words.insert("*", Word::Native(mul));
    words.insert("/", Word::Native(div));
    words.insert("%", Word::Native(rem));
}

pub fn add(run: &mut Runtime) -> anyhow::Result<()> {
    let a = run.stack.pop().ok_or(RuntimeError::StackUnderflow())?;
    let b = run.stack.pop().ok_or(RuntimeError::StackUnderflow())?;
    if let (Value::Number(a), Value::Number(b)) = (a, b) {
        run.stack.push(Value::Number(a + b));
    }
    Ok(())
}

pub fn sub(run: &mut Runtime) -> anyhow::Result<()> {
    let a = run.stack.pop().ok_or(RuntimeError::StackUnderflow())?;
    let b = run.stack.pop().ok_or(RuntimeError::StackUnderflow())?;
    if let (Value::Number(a), Value::Number(b)) = (a, b) {
        run.stack.push(Value::Number(b - a));
    }
    Ok(())
}

pub fn mul(run: &mut Runtime) -> anyhow::Result<()> {
    let a = run.stack.pop().ok_or(RuntimeError::StackUnderflow())?;
    let b = run.stack.pop().ok_or(RuntimeError::StackUnderflow())?;
    if let (Value::Number(a), Value::Number(b)) = (a, b) {
        run.stack.push(Value::Number(a * b));
    }
    Ok(())
}

pub fn div(run: &mut Runtime) -> anyhow::Result<()> {
    let a = run.stack.pop().ok_or(RuntimeError::StackUnderflow())?;
    let b = run.stack.pop().ok_or(RuntimeError::StackUnderflow())?;
    if let (Value::Number(a), Value::Number(b)) = (a, b) {
        run.stack.push(Value::Number(b / a));
    }
    Ok(())
}

pub fn rem(run: &mut Runtime) -> anyhow::Result<()> {
    let a = run.stack.pop().ok_or(RuntimeError::StackUnderflow())?;
    let b = run.stack.pop().ok_or(RuntimeError::StackUnderflow())?;
    if let (Value::Number(a), Value::Number(b)) = (a, b) {
        run.stack.push(Value::Number(b % a));
    }
    Ok(())
}