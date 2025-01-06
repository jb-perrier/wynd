use super::Word;

pub struct Namespace {
    name: String,
    sub_namespaces: Vec<Namespace>,
    words: Vec<Word>,
}