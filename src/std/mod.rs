pub mod math;
pub mod word;

use crate::{instructions::EXIT, Module, Word, WordBuilder};

pub fn modules() -> Vec<Module> {
    let mut root = Module::new("std");
    root.words_mut().extend(words());
    
    vec![
        root,
        math::module(),
    ]
}

pub fn words() -> Vec<Word> {
    vec![WordBuilder::new("exit").builtin(EXIT).build()]
}
