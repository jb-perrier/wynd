use std::collections::HashMap;

use thiserror::Error;

use crate::{Module, Word};

pub struct Symbol {
    name: String,
    address: usize,
}

impl Symbol {
    pub fn new(name: String, address: usize) -> Symbol {
        Symbol { name, address }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn address(&self) -> usize {
        self.address
    }
}
pub struct Image {
    bytecode: Vec<usize>,
    symbols: Vec<Symbol>,
}

impl Image {
    pub fn new() -> Image {
        Image {
            bytecode: Vec::new(),
            symbols: Vec::new(),
        }
    }

    pub fn bytecode(&self) -> &[usize] {
        &self.bytecode
    }

    pub fn push_word(&mut self, word: Word) {
        match word.implem {
            crate::Type::Bytecode(bytecode) => {
                match self.symbols.iter().find(|s| s.name() == word.name) {
                    Some(symbol) => {
                        panic!(
                            "Symbol {} already defined at address {}",
                            symbol.name(),
                            symbol.address()
                        );
                    }
                    None => {
                        let address = self.bytecode.len();
                        self.bytecode.extend(bytecode);
                        self.symbols
                            .push(Symbol::new(word.name.to_string(), address));
                    }
                }
            }
            _ => {}
        }
    }

    pub fn symbols(&self) -> &[Symbol] {
        &self.symbols
    }
}

pub struct ImageBuilder {
    words: Vec<Word>,
}

impl ImageBuilder {
    pub fn new() -> ImageBuilder {
        ImageBuilder { words: Vec::new() }
    }

    pub fn has_word(&self, name: &str) -> bool {
        self.words.iter().any(|s| s.name() == name)
    }

    pub fn word(&mut self, word: Word, modules: &[Module]) {
        for tok in word.tokens() {

        }
    }

    pub fn build(self) -> Image {
        let mut bytecode = Vec::new();
        let mut symbols = Vec::new();
    }
}

#[derive(Debug, Error)]
pub enum ImageError {
    #[error("Symbol {0} already defined at address {1}")]
    SymbolAlreadyDefined(String, usize),
}
