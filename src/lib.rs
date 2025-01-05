#![feature(test)]

use ::std::{
    any::Any, collections::HashMap, fmt::Display, str::FromStr
};
use slab::Slab;

mod error;
mod executor;
mod std;
mod tokenizer;
mod word;
mod image;
mod bench;
mod compiler;

pub use bench::*;
pub use image::*;
pub use error::*;
pub use executor::*;
pub use std::*;
pub use tokenizer::*;
pub use word::*;
pub use compiler::*;

pub type NativeWordFn = fn(&mut Runtime) -> anyhow::Result<()>;

#[derive(Debug, Clone, PartialEq)]
pub enum ValueType {
    Number,
    String,
    List,
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
            Self::List => "list",
        }
    }

    pub fn id(&self) -> usize {
        match self {
            Self::Number => 1,
            Self::String => 2,
            Self::List => 3,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    List(Vec<Value>),
}

impl Display for Value {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Value::Number(num) => write!(f, "{}", num),
            Value::String(string) => write!(f, "{}", string),
            Value::List(list) => {
                write!(f, "[")?;
                for (i, value) in list.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", value)?;
                }
                write!(f, "]")
            }
        }
    }
}

impl Value {
    pub fn value_type(&self) -> ValueType {
        match self {
            Value::Number(_) => ValueType::Number,
            Value::String(_) => ValueType::String,
            Value::List(_) => ValueType::List,
        }
    }

    pub fn as_number(&self) -> Option<f64> {
        match self {
            Value::Number(num) => Some(*num),
            _ => None,
        }
    }

    pub unsafe fn as_number_unchecked(&self) -> f64 {
        match self {
            Value::Number(num) => *num,
            _ => {
                let x: &void::Void = ::std::mem::transmute(1usize);
                void::unreachable(*x)
            }
        }
    }

    pub fn as_usize(&self) -> Option<usize> {
        match self {
            Value::Number(num) => Some(num.to_bits() as usize),
            _ => None,
        }
    }

    pub unsafe fn as_usize_unchecked(&self) -> usize {
        match self {
            Value::Number(num) => num.to_bits() as usize,
            _ => {
                let x: &void::Void = ::std::mem::transmute(1usize);
                void::unreachable(*x)
            }
        }
    }

    pub fn as_string(&self) -> Option<&str> {
        match self {
            Value::String(string) => Some(string),
            _ => None,
        }
    }

    pub fn as_list(&self) -> Option<&[Value]> {
        match self {
            Value::List(list) => Some(list),
            _ => None,
        }
    }
}
pub enum InterpreterError {}

#[derive(Debug)]
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

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            words: Slab::with_capacity(capacity),
            names: HashMap::with_capacity(capacity),
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

pub fn print_stack(stack: &[Value]) {
    for value in stack.iter() {
        match value {
            Value::Number(num) => {
                print!("{} ", num);
            }
            Value::String(string) => {
                print!("\"{}\" ", string);
            }
            Value::List(list) => {
                print!("[ ");
                print_values(list);
                print!("] ");
            }
        }
    }
    println!()
}

pub fn print_values(list: &[Value]) {
    for (i, value) in list.iter().enumerate() {
        if i > 0 && list[i - 1].value_type() != ValueType::List {
            print!(" ");
        }
        match value {
            Value::Number(num) => {
                print!("{}", num);
            }
            Value::String(string) => {
                print!("\"{}\"", string);
            }
            Value::List(list) => {
                print!("[");
                print_values(list);
                print!("]");
            }
        }
    }
}