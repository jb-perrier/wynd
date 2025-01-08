use crate::{instructions, Runtime, RuntimeError, Value, ValueType, WordBuilder};

pub fn insert_math(words: &mut crate::Words) {
    words.insert(
        WordBuilder::new("+")
            .instruction(instructions::ADD)
            .input(ValueType::Number, "Left operand")
            .input(ValueType::Number, "Right operand")
            .output(ValueType::Number, "Result")
            .description("Add two numbers")
            .build(),
    );
    words.insert(
        WordBuilder::new("-")
            .instruction(instructions::SUB)
            .input(ValueType::Number, "Left operand")
            .input(ValueType::Number, "Right operand")
            .output(ValueType::Number, "Result")
            .description("Substract two numbers")
            .build(),
    );
    words.insert(
        WordBuilder::new("*")
            .instruction(instructions::MUL)
            .input(ValueType::Number, "Left operand")
            .input(ValueType::Number, "Right operand")
            .output(ValueType::Number, "Result")
            .description("Multiply two numbers")
            .build(),
    );
    words.insert(
        WordBuilder::new("/")
            .instruction(instructions::DIV)
            .input(ValueType::Number, "Left operand")
            .input(ValueType::Number, "Right operand")
            .output(ValueType::Number, "Result")
            .description("Divide two numbers")
            .build(),
    );
    words.insert(
        WordBuilder::new("%")
            .instruction(instructions::MOD)
            .input(ValueType::Number, "Left operand")
            .input(ValueType::Number, "Right operand")
            .output(ValueType::Number, "Result")
            .description("Remainder of two numbers")
            .build(),
    );
}

#[inline]
pub fn add(runtime: &mut Runtime) -> Result<(), RuntimeError> {
    let a = runtime.peek_value(0);
    let a = *a
        .as_number()
        .ok_or(RuntimeError::UnexpectedValueType(a.type_name().to_owned()))?;

    let b = runtime.peek_value(1);
    let b = *b
        .as_number()
        .ok_or(RuntimeError::UnexpectedValueType(b.type_name().to_owned()))?;

    let result = a + b;

    runtime.pop_value();
    runtime.pop_value();
    runtime.push_value(Value::Number(result));

    Ok(())
}

#[inline]
pub fn sub(runtime: &mut Runtime) -> Result<(), RuntimeError> {
    let a = runtime.peek_value(0);
    let a = *a
        .as_number()
        .ok_or(RuntimeError::UnexpectedValueType(a.type_name().to_owned()))?;

    let b = runtime.peek_value(1);
    let b = *b
        .as_number()
        .ok_or(RuntimeError::UnexpectedValueType(b.type_name().to_owned()))?;

    let result = b - a;

    runtime.pop_value();
    runtime.pop_value();
    runtime.push_value(Value::Number(result));

    Ok(())
}

#[inline]
pub fn mul(runtime: &mut Runtime) -> Result<(), RuntimeError> {
    let a = runtime.peek_value(0);
    let a = *a
        .as_number()
        .ok_or(RuntimeError::UnexpectedValueType(a.type_name().to_owned()))?;

    let b = runtime.peek_value(1);
    let b = *b
        .as_number()
        .ok_or(RuntimeError::UnexpectedValueType(b.type_name().to_owned()))?;

    let result = a * b;

    runtime.pop_value();
    runtime.pop_value();
    runtime.push_value(Value::Number(result));

    Ok(())
}

#[inline]
pub fn div(runtime: &mut Runtime) -> Result<(), RuntimeError> {
    let a = runtime.peek_value(0);
    let a = *a
        .as_number()
        .ok_or(RuntimeError::UnexpectedValueType(a.type_name().to_owned()))?;

    let b = runtime.peek_value(1);
    let b = *b
        .as_number()
        .ok_or(RuntimeError::UnexpectedValueType(b.type_name().to_owned()))?;

    let result = b / a;

    runtime.pop_value();
    runtime.pop_value();
    runtime.push_value(Value::Number(result));

    Ok(())
}

#[inline]
pub fn rem(runtime: &mut Runtime) -> Result<(), RuntimeError> {
    let a = runtime.peek_value(0);
    let a = *a
        .as_number()
        .ok_or(RuntimeError::UnexpectedValueType(a.type_name().to_owned()))?;

    let b = runtime.peek_value(1);
    let b = *b
        .as_number()
        .ok_or(RuntimeError::UnexpectedValueType(b.type_name().to_owned()))?;

    let result = b % a;

    runtime.pop_value();
    runtime.pop_value();
    runtime.push_value(Value::Number(result));

    Ok(())
}
