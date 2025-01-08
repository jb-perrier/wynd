use thiserror::Error;

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("Unknown word: {0}")]
    UnknownWord(String),

    #[error("Unexpected value type: {0}")]
    UnexpectedValueType(String),

    #[error("Invalid opcode")]
    InvalidOpcode,

    #[error("Stack under/over flow, index: {0}")]
    StackOverflow(usize),

    #[error("Frame under/over flow : ({0}, {1})")]
    FrameOverflow(usize, usize),

    #[error("Out of bound word access, index: {0}")]
    OutOfBoundWordAccess(usize),

    #[error("Label not found, id: {0}")]
    LabelNotFound(usize),
}
