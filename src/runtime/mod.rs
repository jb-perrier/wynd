mod bytecode;
mod value;
mod runtime;
mod errors;
mod interpreter;

pub use interpreter::*;
pub use errors::*;
pub use runtime::*;
pub use value::*;
pub use bytecode::*;

pub type RuntimeWordFn = fn(&mut InterpretedWordParameters) -> Result<(), RuntimeError>;