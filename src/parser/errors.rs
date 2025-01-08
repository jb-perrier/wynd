use clap::error;
use thiserror::Error;

use crate::Token;

#[derive(Debug, Error)]
pub enum ParsingError {
    #[error("Unexpected token, actual: {0}")]
    UnexpectedToken(Token),
}