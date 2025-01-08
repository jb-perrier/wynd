mod bytecode;
mod value;
mod runtime;
mod words;
mod errors;

pub use errors::*;
pub use words::*;
pub use runtime::*;
pub use value::*;
pub use bytecode::*;

pub type RuntimeWordFn = fn(&mut Runtime) -> Result<(), RuntimeError>;