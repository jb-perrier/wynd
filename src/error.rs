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
}