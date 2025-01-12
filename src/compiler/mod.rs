mod errors;
mod image;
pub mod instructions;

pub use errors::*;
pub use image::*;

use instructions::{CALL_BYTECODE, CALL_NATIVE, LOAD_NUMBER};

use crate::{Module, Token};

pub fn compile_to_image(
    toks: &[Token],
    modules: &[Module],
    image: &mut Image,
) -> anyhow::Result<()> {
    let mut bytecode = Vec::new();

    for tok in toks {
        match tok {
            Token::Word(name) => {
                if let Some((index, word)) = words.get_by_name(name) {
                    match word.implem {
                        crate::Type::Native(func) => {
                            bytecode.push(CALL_NATIVE);
                            bytecode.push(func as usize);
                        }
                        crate::Type::Bytecode(_) => {
                            bytecode.push(CALL_BYTECODE);
                            bytecode.push(index);
                        }
                        crate::Type::Builtin(id) => {
                            bytecode.push(id);
                        }
                    }
                } else {
                    return Err(CompilerError::UnknownWord(name.to_string()).into());
                }
            }
            Token::Number(num) => {
                bytecode.push(LOAD_NUMBER);
                bytecode.push(num.to_bits() as usize);
            }
            _ => unimplemented!(),
        }
    }

    Ok(())
}
