use crate::{instructions::{self, ADD, DIV, MOD, MUL, SUB}, InterpretedWordParameters, RuntimeError, Value, ValueType, Word, WordBuilder};

pub fn module() -> crate::Module {
    let mut module = crate::Module::new("math");
    module.words_mut().extend(words());
    module
}

pub fn words() -> Vec<Word> {
    vec![
        WordBuilder::new("+")
            .builtin(ADD)
            .input(ValueType::Number, "Left operand")
            .input(ValueType::Number, "Right operand")
            .output(ValueType::Number, "Result")
            .description("Add two numbers")
            .build(),
        WordBuilder::new("-")
            .builtin(SUB)
            .input(ValueType::Number, "Left operand")
            .input(ValueType::Number, "Right operand")
            .output(ValueType::Number, "Result")
            .description("Substract two numbers")
            .build(),
        WordBuilder::new("*")
            .builtin(MUL)
            .input(ValueType::Number, "Left operand")
            .input(ValueType::Number, "Right operand")
            .output(ValueType::Number, "Result")
            .description("Multiply two numbers")
            .build(),
        WordBuilder::new("/")
            .builtin(DIV)
            .input(ValueType::Number, "Left operand")
            .input(ValueType::Number, "Right operand")
            .output(ValueType::Number, "Result")
            .description("Divide two numbers")
            .build(),
        WordBuilder::new("%")
            .builtin(MOD)
            .input(ValueType::Number, "Left operand")
            .input(ValueType::Number, "Right operand")
            .output(ValueType::Number, "Result")
            .description("Remainder of two numbers")
            .build(),
    ]
}

#[inline]
pub fn add(runtime: &mut InterpretedWordParameters) -> Result<(), RuntimeError> {
    let b = runtime.values_mut().pop().unwrap();
    let b = *b
        .as_number()
        .ok_or_else(|| RuntimeError::UnexpectedValueType(b.type_name().to_owned()))?;

    let a = runtime.values_mut().pop().unwrap();
    let a = *a
        .as_number()
        .ok_or_else(|| RuntimeError::UnexpectedValueType(a.type_name().to_owned()))?;

    // println!("ADD: {} + {}", a, b);
    let result = a + b;

    runtime.values_mut().push(Value::Number(result));
    runtime.ip_add(1);
    Ok(())
}

#[inline]
pub fn sub(runtime: &mut InterpretedWordParameters) -> Result<(), RuntimeError> {
    let a = runtime.values_mut().pop().unwrap();
    let a = *a
        .as_number()
        .ok_or_else(|| RuntimeError::UnexpectedValueType(a.type_name().to_owned()))?;

    let b = runtime.values_mut().pop().unwrap();
    let b = *b
        .as_number()
        .ok_or_else(|| RuntimeError::UnexpectedValueType(b.type_name().to_owned()))?;

    // println!("SUB: {} - {}", a, b);
    let result = b - a;

    runtime.values_mut().push(Value::Number(result));
    runtime.ip_add(1);
    Ok(())
}

#[inline]
pub fn mul(runtime: &mut InterpretedWordParameters) -> Result<(), RuntimeError> {
    let a = runtime.values_mut().pop().unwrap();
    let a = *a
        .as_number()
        .ok_or_else(|| RuntimeError::UnexpectedValueType(a.type_name().to_owned()))?;

    let b = runtime.values_mut().pop().unwrap();
    let b = *b
        .as_number()
        .ok_or_else(|| RuntimeError::UnexpectedValueType(b.type_name().to_owned()))?;

    let result = a * b;

    runtime.values_mut().push(Value::Number(result));
    runtime.ip_add(1);
    Ok(())
}

#[inline]
pub fn div(runtime: &mut InterpretedWordParameters) -> Result<(), RuntimeError> {
    let a = runtime.values_mut().pop().unwrap();
    let a = *a
        .as_number()
        .ok_or_else(|| RuntimeError::UnexpectedValueType(a.type_name().to_owned()))?;

    let b = runtime.values_mut().pop().unwrap();
    let b = *b
        .as_number()
        .ok_or_else(|| RuntimeError::UnexpectedValueType(b.type_name().to_owned()))?;

    let result = b / a;

    runtime.values_mut().push(Value::Number(result));
    runtime.ip_add(1);
    Ok(())
}

#[inline]
pub fn rem(runtime: &mut InterpretedWordParameters) -> Result<(), RuntimeError> {
    let a = runtime.values_mut().pop().unwrap();
    let a = *a
        .as_number()
        .ok_or_else(|| RuntimeError::UnexpectedValueType(a.type_name().to_owned()))?;

    let b = runtime.values_mut().pop().unwrap();
    let b = *b
        .as_number()
        .ok_or_else(|| RuntimeError::UnexpectedValueType(b.type_name().to_owned()))?;

    let result = b % a;

    runtime.values_mut().push(Value::Number(result));
    runtime.ip_add(1);
    Ok(())
}
