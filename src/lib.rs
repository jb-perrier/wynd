use ::std::collections::HashMap;

mod tokenizer;
mod executor;
mod std;
mod error;

use slab::Slab;

pub use tokenizer::*;
pub use executor::*;
pub use std::*;
pub use error::*;

pub type NativeWordFn = fn(&mut Runtime) -> Result<(), anyhow::Error>;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
}

pub enum InterpreterError {

}

pub enum Word {
    Source(Vec<Token>),
    Native(NativeWordFn),
}

pub struct Words {
    words: Slab<Word>,
    names: HashMap<String, usize>,
}

impl Default for Words {
    fn default() -> Self {
        Self::new()
    }
}

impl Words {
    pub fn new() -> Self {
        Self {
            words: Slab::new(),
            names: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: &str, word: Word) -> usize {
        let id = self.words.insert(word);
        self.names.insert(name.to_string(), id);
        id
    }
    
    pub fn get(&self, name: &str) -> Option<&Word> {
        self.names.get(name).and_then(|id| self.words.get(*id))
    }

    pub fn get_by_index(&self, index: usize) -> Option<&Word> {
        self.words.get(index)
    }

    pub fn get_index(&self, name: &str) -> Option<usize> {
        self.names.get(name).copied()
    }

    pub fn remove(&mut self, name: &str) {
        if let Some(id) = self.names.remove(name) {
            self.words.remove(id);
        }
    }

    pub fn clear(&mut self) {
        self.words.clear();
        self.names.clear();
    }
}

pub fn print_stack(stack: &mut [Value]) {
    for value in stack.iter() {
        match value {
            Value::Number(num) => {
                print!("{} ", num);
            }
            Value::String(string) => {
                print!("{} ", string);
            }
        }
    }
    println!()
}