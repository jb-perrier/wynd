use crate::Words;

mod math;
mod stack;

pub use math::*;
pub use stack::*;

pub fn insert_std(words: &mut Words) {
    insert_math(words);
    insert_stack(words);
}