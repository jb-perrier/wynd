// use anyhow::anyhow;
// use instructions::{ADD, CALL, DIV, EXIT, JMP_LBL, LBL, LOAD, MOD, MUL, RET, SUB};
// use thiserror::Error;

// use crate::{word, NativeWordFn, Token, Value, WordCode, Words};

// pub fn compile_to_bytecode(toks: &[Token]) -> anyhow::Result<Vec<usize>> {
//     let mut bytecode = Vec::new();

//     for tok in toks {
//         match tok {
//             Token::Word(name) => match name.as_str() {
//                 // builtin words
//                 "+" => bytecode.push(ADD),
//                 "-" => bytecode.push(SUB),
//                 "*" => bytecode.push(MUL),
//                 "/" => bytecode.push(DIV),
//                 "%" => bytecode.push(MOD),
//                 "ret" => bytecode.push(RET),
//                 "exit" => bytecode.push(EXIT),
//                 name => {}
//             },
//             Token::Number(num) => {
//                 bytecode.push(LOAD);
//                 bytecode.push(num.to_bits() as usize);
//             }
//             _ => todo!(),
//         }
//     }

//     Ok(bytecode)
// }



// mod instructions {
//     pub const ADD: usize = 0;
//     pub const SUB: usize = 1;
//     pub const MUL: usize = 2;
//     pub const DIV: usize = 3;
//     pub const MOD: usize = 4;
//     pub const LOAD: usize = 5;
//     pub const CALL: usize = 6;
//     pub const RET: usize = 7;
//     pub const EXIT: usize = 8;
//     pub const LBL: usize = 9;
//     pub const JMP_LBL: usize = 10;
// }
