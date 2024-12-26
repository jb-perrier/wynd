use crate::{Runtime, Value, ValueType, WordBuilder, WordCode, Words};

pub fn insert_io(words: &mut Words) {

    words.insert(WordBuilder::new("print").input(ValueType::String, "The string to print").description("Print a string").code(WordCode::Native(print)).build());
    words.insert(WordBuilder::new("println").input(ValueType::String, "The string to print").description("Print a string with a newline").code(WordCode::Native(println)).build());
    words.insert(WordBuilder::new("read").output(ValueType::String, "The input string").description("Read a line from stdin").code(WordCode::Native(read)).build());
}

pub fn print(run: &mut Runtime) -> anyhow::Result<()> {
    let value = run.stack.pop().ok_or(crate::RuntimeError::StackUnderflow())?;
    print!("{}", value);
    Ok(())
}

pub fn println(run: &mut Runtime) -> anyhow::Result<()> {
    let value = run.stack.pop().ok_or(crate::RuntimeError::StackUnderflow())?;
    println!("{}", value);
    Ok(())
}

pub fn read(run: &mut Runtime) -> anyhow::Result<()> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    run.stack.push(Value::String(input.trim().to_string()));
    Ok(())
}