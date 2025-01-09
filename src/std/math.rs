use crate::{instructions, Runtime, RuntimeError, Value, ValueType, WordBuilder};

pub fn insert_math(words: &mut crate::Words) {
    words.insert(
        WordBuilder::new("+")
            .builtin(instructions::ADD)
            .input(ValueType::Number, "Left operand")
            .input(ValueType::Number, "Right operand")
            .output(ValueType::Number, "Result")
            .description("Add two numbers")
            .build(),
    );
    words.insert(
        WordBuilder::new("-")
            .builtin(instructions::SUB)
            .input(ValueType::Number, "Left operand")
            .input(ValueType::Number, "Right operand")
            .output(ValueType::Number, "Result")
            .description("Substract two numbers")
            .build(),
    );
    words.insert(
        WordBuilder::new("*")
            .builtin(instructions::MUL)
            .input(ValueType::Number, "Left operand")
            .input(ValueType::Number, "Right operand")
            .output(ValueType::Number, "Result")
            .description("Multiply two numbers")
            .build(),
    );
    words.insert(
        WordBuilder::new("/")
            .builtin(instructions::DIV)
            .input(ValueType::Number, "Left operand")
            .input(ValueType::Number, "Right operand")
            .output(ValueType::Number, "Result")
            .description("Divide two numbers")
            .build(),
    );
    words.insert(
        WordBuilder::new("%")
            .builtin(instructions::MOD)
            .input(ValueType::Number, "Left operand")
            .input(ValueType::Number, "Right operand")
            .output(ValueType::Number, "Result")
            .description("Remainder of two numbers")
            .build(),
    );
}

#[inline]
pub fn add(runtime: &mut Runtime) -> Result<(), RuntimeError> {
    let b = runtime.peek_value(0);
    let b = *b
        .as_number()
        .ok_or_else(|| RuntimeError::UnexpectedValueType(b.type_name().to_owned()))?;

    let a = runtime.peek_value(1);
    let a = *a
        .as_number()
        .ok_or_else(|| RuntimeError::UnexpectedValueType(a.type_name().to_owned()))?;

    // println!("ADD: {} + {}", a, b);
    let result = a + b;

    runtime.pop_value();
    runtime.pop_value();
    runtime.push_value(Value::Number(result));

    runtime.frame_add(1);
    Ok(())
}

#[inline]
pub fn sub(runtime: &mut Runtime) -> Result<(), RuntimeError> {
    let a = runtime.peek_value(0);
    let a = *a
        .as_number()
        .ok_or_else(|| RuntimeError::UnexpectedValueType(a.type_name().to_owned()))?;

    let b = runtime.peek_value(1);
    let b = *b
        .as_number()
        .ok_or_else(|| RuntimeError::UnexpectedValueType(b.type_name().to_owned()))?;

    // println!("SUB: {} - {}", a, b);
    let result = b - a;

    runtime.pop_value();
    runtime.pop_value();
    runtime.push_value(Value::Number(result));

    runtime.frame_add(1);
    Ok(())
}

#[inline]
pub fn mul(runtime: &mut Runtime) -> Result<(), RuntimeError> {
    let a = runtime.peek_value(0);
    let a = *a
        .as_number()
        .ok_or_else(|| RuntimeError::UnexpectedValueType(a.type_name().to_owned()))?;

    let b = runtime.peek_value(1);
    let b = *b
        .as_number()
        .ok_or_else(|| RuntimeError::UnexpectedValueType(b.type_name().to_owned()))?;

    let result = a * b;

    runtime.pop_value();
    runtime.pop_value();
    runtime.push_value(Value::Number(result));

    runtime.frame_add(1);
    Ok(())
}

#[inline]
pub fn div(runtime: &mut Runtime) -> Result<(), RuntimeError> {
    let a = runtime.peek_value(0);
    let a = *a
        .as_number()
        .ok_or_else(|| RuntimeError::UnexpectedValueType(a.type_name().to_owned()))?;

    let b = runtime.peek_value(1);
    let b = *b
        .as_number()
        .ok_or_else(|| RuntimeError::UnexpectedValueType(b.type_name().to_owned()))?;

    let result = b / a;

    runtime.pop_value();
    runtime.pop_value();
    runtime.push_value(Value::Number(result));

    runtime.frame_add(1);
    Ok(())
}

#[inline]
pub fn rem(runtime: &mut Runtime) -> Result<(), RuntimeError> {
    let a = runtime.peek_value(0);
    let a = *a
        .as_number()
        .ok_or_else(|| RuntimeError::UnexpectedValueType(a.type_name().to_owned()))?;

    let b = runtime.peek_value(1);
    let b = *b
        .as_number()
        .ok_or_else(|| RuntimeError::UnexpectedValueType(b.type_name().to_owned()))?;

    let result = b % a;

    runtime.pop_value();
    runtime.pop_value();
    runtime.push_value(Value::Number(result));

    runtime.frame_add(1);
    Ok(())
}
