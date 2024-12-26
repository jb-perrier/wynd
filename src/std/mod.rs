use crate::{Runtime, WordBuilder, WordCode, Words};

mod io;
mod math;
mod stack;
mod word;
mod string;
mod number;
mod shell;
mod help;

pub use help::*;
pub use shell::*;
pub use number::*;
pub use string::*;
pub use io::*;
pub use math::*;
pub use stack::*;
pub use word::*;

pub fn insert_std(words: &mut Words) {
    insert_math(words);
    insert_stack(words);
    insert_word(words);
    insert_io(words);
    insert_string(words);
    insert_number(words);
    insert_shell(words);
    insert_help(words);

    words.insert(WordBuilder::new("cast").code(WordCode::Native(cast)).description("Cast the top value on the stack to a different type").build());
}

pub fn cast(run: &mut Runtime) -> anyhow::Result<()> {
    let type_id = run.stack.pop().ok_or(crate::RuntimeError::StackUnderflow())?;
    let type_id = type_id.as_number().ok_or(crate::RuntimeError::UnexpectedValue { expected: "number".into(), found: type_id.value_type().as_str().into() })?;

    match type_id {
        1.0 => {
            to_number(run)?;
        }
        2.0 => {
            to_string(run)?;
        }
        _ => {
            return Err(crate::RuntimeError::UnexpectedValue { expected: "number".to_string(), found: type_id.to_string() }.into());
        }
    }
    Ok(())
}