use clap::error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("stack underflow")]
    StackUnderflow(),

    #[error("stack overflow")]
    StackOverflow(),

    #[error("unknown word \"{name}\"")]
    UnknownWord {
        name: String,
    },

    #[error("unknown word \"{id}\"")]
    UnknownWordId {
        id: usize,
    },

    #[error("value error: expected {expected}, found {found}")]
    UnexpectedValue {
        expected: String,
        found: String,
    },

    #[error("missing function name")]
    MissingFunctionName,

    #[error("cannot convert {from} to {to}")]
    CannotConvertValue {
        from: &'static str,
        to: &'static str,
    },

    #[error("word not allowed in list")]
    WordNotAllowedInList,
}