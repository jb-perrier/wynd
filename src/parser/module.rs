use crate::std::word::{FindWord, FindWordMut};

use super::Word;

pub struct Module {
    name: String,
    words: Vec<Word>,
}

impl Module {
    pub fn new<T: Into<String>>(name: T) -> Module {
        Module {
            name: name.into(),
            words: Vec::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn words(&self) -> &[Word] {
        &self.words
    }

    pub fn words_mut(&mut self) -> &mut Vec<Word> {
        &mut self.words
    }
}

pub fn split_module_path(path: &str) -> Vec<&str> {
    path.split('.').collect()
}


pub trait FindModule {
    fn find_module(&self, name: &str) -> Option<&Module>;
}

pub trait FindModuleMut {
    fn find_module_mut(&mut self, name: &str) -> Option<&mut Module>;
}

impl FindModule for &[Module] {
    fn find_module(&self, name: &str) -> Option<&Module> {
        self.iter().find(|m| m.name() == name)
    }
}

impl FindModuleMut for &mut [Module] {
    fn find_module_mut(&mut self, name: &str) -> Option<&mut Module> {
        self.iter_mut().find(|m| m.name() == name)
    }
}

impl FindWord for &[Module] {
    fn find_word(&self, name: &str) -> Option<&Word> {
        for module in self.iter() {
            if let Some(word) = module.words().iter().find(|word| word.name() == name) {
                return Some(word);
            }
        }
        None
    }
}

impl FindWordMut for &mut [Module] {
    fn find_word_mut(&mut self, name: &str) -> Option<&mut Word> {
        for module in self.iter_mut() {
            if let Some(word) = module.words_mut().iter_mut().find(|word| word.name() == name) {
                return Some(word);
            }
        }
        None
    }
}