#![feature(test)]

use ::std::{
    any::Any, collections::HashMap, fmt::Display, str::FromStr
};
use slab::Slab;

mod parser;
mod runtime;
mod tokenizer;
mod bench;
mod compiler;
pub mod std;
mod utils;

pub use utils::*;
pub use parser::*;
pub use runtime::*;
pub use tokenizer::*;
pub use compiler::*;

pub fn rust_compute() -> i32 {
    let mut result = 1 + 2;
        result = result + 6;
        result = result - 10;
        result = result * 2;
        result = result / 2;
        result = result + 6;
        result = result - 10;
        result = result * 2;
        result = result / 6;
        result = result / 9;
        result = result * 2;
        result = result + 6;
        result = result - 10;
        result = result * 2;
        result = result / 2;
        result = result + 6;
        result = result - 10;
        result = result * 2;
        result = result / 6;
        result = result / 9;
        result = result + 1 + 2;
        result = result + 6;
        result = result - 10;
        result = result * 2;
        result = result / 2;
        result = result + 6;
        result = result - 10;
        result = result * 2;
        result = result / 6;
        result = result / 9;
        result = result * 2;
        result = result + 6;
        result = result - 10;
        result = result * 2;
        result = result / 2;
        result = result + 6;
        result = result - 10;
        result = result * 2;
        result = result / 6;
        result = result / 9;
        result
}