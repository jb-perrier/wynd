use crate::{Runtime, RuntimeError, Word};

pub fn insert_stack(words: &mut crate::Words) {
    words.insert("drop", Word::Native(drop));
    words.insert("dup", Word::Native(dup));
    words.insert("stack", Word::Native(print_stack));
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