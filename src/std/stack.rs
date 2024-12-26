use crate::{Runtime, RuntimeError, WordBuilder, WordCode};

pub fn insert_stack(words: &mut crate::Words) {
    words.insert(WordBuilder::new("stack").code(WordCode::Native(print_stack)).description("Print the stack").build());
    words.insert(WordBuilder::new("drop").code(WordCode::Native(drop)).description("Drop the top value from the stack").build());
    words.insert(WordBuilder::new("dup").code(WordCode::Native(dup)).description("Duplicate the top value on the stack").build());
}

pub fn print_stack(run: &mut Runtime) -> anyhow::Result<()> {
    crate::print_stack(run.stack);
    Ok(())
}

pub fn drop(run: &mut Runtime) -> anyhow::Result<()> {
    run.stack.pop().ok_or(RuntimeError::StackUnderflow())?;
    Ok(())
}

pub fn dup(run: &mut Runtime) -> anyhow::Result<()> {
    let a = run.stack.last().ok_or(RuntimeError::StackUnderflow())?;
    run.stack.push(a.clone());
    Ok(())
}