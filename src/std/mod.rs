pub mod math;

use crate::Words;

pub fn insert_std(words: &mut Words) {
    math::insert_math(words);
}
