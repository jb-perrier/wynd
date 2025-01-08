use std::collections::HashMap;

use slab::Slab;

use crate::Word;

#[derive(Debug, Clone, Copy)]
pub struct InstructionAddress {
    pub word: usize,
    pub instr: usize,
}

pub struct Words {
    words: Slab<Word>,
    names: HashMap<String, usize>,
}

impl Words {
    pub fn get_instruction(&self, instr_addr: &InstructionAddress) -> Option<usize> {
        self.words.get(instr_addr.word).and_then(|word| {
            word.implem.as_virtual().map(|v| v[instr_addr.instr])
        })
    }

    pub fn get_by_name(&self, name: &str) -> Option<&Word> {
        self.names.get(name).map(|&id| &self.words[id])
    }

    pub fn get(&self, id: usize) -> Option<&Word> {
        self.words.get(id)
    }
    pub fn get_index_by_name(&self, name: &str) -> Option<usize> {
        self.names.get(name).copied()
    }

    pub fn insert(&mut self, word: Word) {
        let name = word.name.clone();
        let id = self.words.insert(word);
        self.names.insert(name, id);
    }
}
