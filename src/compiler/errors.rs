use thiserror::Error;

#[derive(Debug, Error)]
pub enum CompilerError {
    #[error("Unknown word: {0}")]
    UnknownWord(String),
}