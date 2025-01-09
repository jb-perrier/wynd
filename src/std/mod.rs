pub mod math;

use crate::{instructions::EXIT, WordBuilder, Words};

pub fn insert_std(words: &mut Words) {
    math::insert_math(words);

    let exit_word = WordBuilder::new("exit")
        .builtin(EXIT)
        .build();
    words.insert(exit_word);
}
