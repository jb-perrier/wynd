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
mod tail_call;

pub use tail_call::*;
pub use parser::*;
pub use runner::*;
pub use tokenizer::*;
pub use compiler::*;