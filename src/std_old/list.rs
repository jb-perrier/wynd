use crate::{Runtime, WordBuilder, WordCode};

pub fn insert_list(words: &mut crate::Words) {
    words.insert(WordBuilder::new("list").code(WordCode::Native(list_type)).description("Put the type id of List on top the stack").build());
}

pub fn list_type(run: &mut Runtime) -> anyhow::Result<()> {
    run.stack.push(crate::Value::Number(3.0));
    Ok(())
}

