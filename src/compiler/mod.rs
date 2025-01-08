pub mod instructions;
mod errors;

pub use errors::*;

use instructions::{ADD, CALL, DIV, EXIT, JMP_LBL, LBL, LOAD, MOD, MUL, RET, SUB};

use crate::{Token, Words};

pub fn compile_to_bytecode(toks: &[Token], words: &mut Words) -> anyhow::Result<Vec<usize>> {
    let mut bytecode = Vec::new();

    for tok in toks {
        match tok {
            Token::Word(name) => match name.as_str() {
                // builtin words
                "+" => bytecode.push(ADD),
                "-" => bytecode.push(SUB),
                "*" => bytecode.push(MUL),
                "/" => bytecode.push(DIV),
                "%" => bytecode.push(MOD),
                "ret" => bytecode.push(RET),
                "exit" => bytecode.push(EXIT),
                name => {
                    if let Some(word) = words.get_index_by_name(name) {
                        bytecode.push(CALL);
                        bytecode.push(word);
                    } else {
                        return Err(CompilerError::UnknownWord(name.to_string()).into());
                    }
                }
            },
            Token::Number(num) => {
                bytecode.push(LOAD);
                bytecode.push(num.to_bits() as usize);
            }
            _ => todo!(),
        }
    }

    Ok(bytecode)
}




