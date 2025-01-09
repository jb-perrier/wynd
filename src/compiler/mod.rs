mod errors;
pub mod instructions;

pub use errors::*;

use instructions::{CALL_BYTECODE, CALL_NATIVE, LOAD_NUMBER};

use crate::{Token, Words};

pub fn compile_to_bytecode(toks: &[Token], words: &mut Words) -> anyhow::Result<Vec<usize>> {
    let mut bytecode = Vec::new();

    for tok in toks {
        match tok {
            Token::Word(name) => {
                if let Some((index, word)) = words.get_by_name(name) {
                    match word.implem {
                        crate::WordImpl::Native(func) => {
                            bytecode.push(CALL_NATIVE);
                            bytecode.push(func as usize);
                        }
                        crate::WordImpl::Bytecode(_) => {
                            bytecode.push(CALL_BYTECODE);
                            bytecode.push(index);
                        }
                        crate::WordImpl::Builtin(id) => {
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

    Ok(bytecode)
}
