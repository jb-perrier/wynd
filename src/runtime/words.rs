use std::collections::HashMap;

use slab::Slab;

use crate::Word;

#[derive(Debug, Default, Clone, Copy)]
pub struct Frame {
    pub word: usize,
    pub bytecode: usize,
}

pub struct Words {
    words: Slab<Word>,
    names: HashMap<String, usize>,
}

impl Words {
    pub fn new() -> Self {
        Self {
            words: Slab::new(),
            names: HashMap::new(),
        }
    }

    pub fn get_bytecode(&self, bytecode_addr: &Frame) -> Option<usize> {
        self.words
            .get(bytecode_addr.word)
            .and_then(|word| word.implem.as_bytecode().map(|v| v[bytecode_addr.bytecode]))
    }

    pub fn get_by_name(&self, name: &str) -> Option<(usize, &Word)> {
        self.names
            .get(name)
            .and_then(|id| self.words.get(*id).map(|w| (*id, w)))
    }

    pub fn get(&self, id: usize) -> Option<&Word> {
        self.words.get(id)
    }
    pub fn get_index_by_name(&self, name: &str) -> Option<usize> {
        self.names.get(name).copied()
    }

    pub fn insert(&mut self, word: Word) -> usize {
        let name = word.name.clone();
        let id = self.words.insert(word);
        self.names.insert(name, id);
        id
    }
}
