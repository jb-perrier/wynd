#![feature(test)]

use ::std::{
    any::Any, collections::HashMap, fmt::Display, str::FromStr
};
use slab::Slab;

mod parser;
mod runner;
mod tokenizer;
mod bench;
mod compiler;

pub use parser::*;
pub use runner::*;
pub use tokenizer::*;
pub use compiler::*;