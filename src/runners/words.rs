use std::collections::HashMap;

use slab::Slab;

use crate::Word;

pub struct Words {
    words: Slab<Word>,
    names: HashMap<String, usize>,
}
