use thiserror::Error;

#[derive(Debug, Error)]
pub enum BytecodeRunnerError {
    #[error("Invalid opcode")]
    InvalidOpcode,
    #[error("Stack under/over flow, index: {0}")]
    StackOverflow(usize),
    #[error("Frame under/over flow, frame index: {0}")]
    FrameOverflow(usize),
    #[error("Out of bound word access, index: {0}")]
    OutOfBoundWordAccess(usize),
    #[error("Label not found, id: {0}")]
    LabelNotFound(usize),
}