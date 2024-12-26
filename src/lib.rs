use ::std::{
    collections::{hash_map::Keys, BTreeMap, HashMap},
    fmt::Display,
    str::FromStr,
};
use slab::Slab;

mod error;
mod executor;
mod std;
mod tokenizer;
mod word;

pub use error::*;
pub use executor::*;
pub use std::*;
pub use tokenizer::*;
pub use word::*;

pub type NativeWordFn = fn(&mut Runtime) -> anyhow::Result<()>;

pub enum ValueType {
    Number,
    String,
}

impl FromStr for ValueType {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "number" => Ok(Self::Number),
            "string" => Ok(Self::String),
            _ => Err(()),
        }
    }
}

impl ValueType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Number => "number",
            Self::String => "string",
        }
    }

    pub fn id(&self) -> usize {
        match self {
            Self::Number => 1,
            Self::String => 2,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
}

impl Display for Value {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Value::Number(num) => write!(f, "{}", num),
            Value::String(string) => write!(f, "{}", string),
        }
    }
}

impl Value {
    pub fn value_type(&self) -> ValueType {
        match self {
            Value::Number(_) => ValueType::Number,
            Value::String(_) => ValueType::String,
        }
    }

    pub fn as_number(&self) -> Option<f64> {
        match self {
            Value::Number(num) => Some(*num),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<&str> {
        match self {
            Value::String(string) => Some(string),
            _ => None,
        }
    }
}
pub enum InterpreterError {}

pub enum WordCode {
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

    pub fn names(&self) -> impl Iterator<Item = (&str, usize)> {
        self.names.iter().map(|(k, v)| (k.as_str(), *v))
    }

    pub fn insert(&mut self, word: Word) -> usize {
        let name = word.name.clone();
        let id = self.words.insert(word);
        self.names.insert(name, id);
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
                print!("\"{}\" ", string);
            }
        }
    }
    println!()
}
