use crate::{Runtime, RuntimeError, Value, ValueType, WordBuilder, WordCode};

pub fn insert_math(words: &mut crate::Words) {
    words.insert(
        WordBuilder::new("+")
            .code(WordCode::Native(add))
            .input(ValueType::Number, "Left operand")
            .input(ValueType::Number, "Right operand")
            .output(ValueType::Number, "Result")
            .description("Add two numbers")
            .build(),
    );
    words.insert(
        WordBuilder::new("-")
            .code(WordCode::Native(sub))
            .input(ValueType::Number, "Left operand")
            .input(ValueType::Number, "Right operand")
            .output(ValueType::Number, "Result")
            .description("Substract two numbers")
            .build(),
    );
    words.insert(
        WordBuilder::new("*")
            .code(WordCode::Native(mul))
            .input(ValueType::Number, "Left operand")
            .input(ValueType::Number, "Right operand")
            .output(ValueType::Number, "Result")
            .description("Multiply two numbers")
            .build(),
    );
    words.insert(
        WordBuilder::new("/")
            .code(WordCode::Native(div))
            .input(ValueType::Number, "Left operand")
            .input(ValueType::Number, "Right operand")
            .output(ValueType::Number, "Result")
            .description("Divide two numbers")
            .build(),
    );
    words.insert(
        WordBuilder::new("%")
            .code(WordCode::Native(rem))
            .input(ValueType::Number, "Left operand")
            .input(ValueType::Number, "Right operand")
            .output(ValueType::Number, "Result")
            .description("Remainder of two numbers")
            .build(),
    );
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
