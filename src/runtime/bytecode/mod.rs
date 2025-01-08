use crate::compiler::instructions::*;

mod errors;

pub use errors::*;

use super::{Runtime, RuntimeError, Value};

pub fn execute_bytecode(runtime: &mut Runtime) -> Result<(), RuntimeError> {
    loop {
        let word_addr = *runtime.frame().unwrap();
        let opcode = *runtime.words().get(word_addr.word).unwrap().implem.as_virtual().unwrap().get(word_addr.instr).unwrap();

        match opcode {
            // add
            ADD => crate::std::math::add(runtime)?,
            // sub
            SUB => crate::std::math::sub(runtime)?,
            // mul
            MUL => crate::std::math::mul(runtime)?,
            // div
            DIV => crate::std::math::div(runtime)?,
            // mod
            MOD => crate::std::math::rem(runtime)?,
            // load
            LOAD => {
                let bytecode = runtime.words().get_instruction(&word_addr).unwrap();
                runtime.push_value(Value::Number(f64::from_bits(bytecode as u64)));

                runtime.frame_incr_instr(2);
            }
            // call
            CALL => {
                // let stack_previous_pos = *stack_pos - 1;
                // let next_word_pos = *stack
                //     .get(*stack_pos - 1)
                //     .ok_or(RuntimeError::StackOverflow(stack_previous_pos))?;

                // // moving current word pos to have the next pos when returning
                // let word_pos = frame_stack
                //     .get_mut(*frame_pos)
                //     .ok_or(RuntimeError::FrameOverflow(*frame_pos))?;
                // *word_pos += 1;

                // // adding frame for the entering the called word
                // *frame_pos += 1;
                // let ret_slot = frame_stack
                //     .get_mut(*frame_pos)
                //     .ok_or(RuntimeError::FrameOverflow(*frame_pos))?;
                // *ret_slot = next_word_pos;
            }
            // ret
            RET => {
                // *frame_pos -= 1;
            }
            // exit
            EXIT => {
                break;
            }
            // label
            LBL => {
                // lbl opcode + id
                // let word_pos = frame_stack
                //     .get_mut(*frame_pos)
                //     .ok_or(RuntimeError::FrameOverflow(*frame_pos))?;
                // *word_pos += 2;
            }
            // jmp_lbl
            JMP_LBL => {
                // let lbl_id = *stack
                //     .get(*stack_pos - 1)
                //     .ok_or(RuntimeError::StackOverflow(*stack_pos - 1))?;
                // let word_pos = frame_stack
                //     .get_mut(*frame_pos)
                //     .ok_or(RuntimeError::FrameOverflow(*frame_pos))?;
                // let mut pos = *word_pos;
                // let mut found = false;
                // while pos < words.len() {
                //     let opcode = *words
                //         .get(pos)
                //         .ok_or(RuntimeError::OutOfBoundWordAccess(pos))?;
                //     if opcode == LBL {
                //         let id = *words
                //             .get(pos + 1)
                //             .ok_or(RuntimeError::OutOfBoundWordAccess(pos + 1))?;
                //         if id == lbl_id {
                //             found = true;
                //             break;
                //         }
                //     }
                //     pos += 2;
                // }

                // if found {
                //     *word_pos = pos;
                // } else {
                //     return Err(RuntimeError::LabelNotFound(lbl_id));
                // }

                // *stack_pos -= 1;
            }
            _ => {
                return Err(RuntimeError::InvalidOpcode);
            }
        }
    }

    Ok(())
}

// pub fn execute_bytecode(
//     stack: &mut [usize],
//     stack_pos: &mut usize,
//     frame_stack: &mut [usize],
//     frame_pos: &mut usize,
//     words: &mut [usize],
// ) -> Result<(), BytecodeRunnerError> {

//     loop {
//         let word_pos = *frame_stack
//             .get(*frame_pos)
//             .ok_or(BytecodeRunnerError::FrameOverflow(*frame_pos))?;
//         let opcode = *words
//             .get(word_pos)
//             .ok_or(BytecodeRunnerError::OutOfBoundWordAccess(word_pos))?;

//         match opcode {
//             // add
//             ADD => {
//                 let a = f64::from_bits(
//                     *stack
//                         .get(*stack_pos - 1)
//                         .ok_or(BytecodeRunnerError::StackOverflow(*stack_pos - 1))?
//                         as u64,
//                 );
//                 let b = f64::from_bits(
//                     *stack
//                         .get(*stack_pos - 2)
//                         .ok_or(BytecodeRunnerError::StackOverflow(*stack_pos - 2))?
//                         as u64,
//                 );
//                 let res = a + b;
//                 let slot = stack
//                     .get_mut(*stack_pos - 2)
//                     .ok_or(BytecodeRunnerError::StackOverflow(*stack_pos - 2))?;
//                 *slot = res.to_bits() as usize;
//                 *stack_pos -= 1;

//                 let word_pos = frame_stack
//                     .get_mut(*frame_pos)
//                     .ok_or(BytecodeRunnerError::FrameOverflow(*frame_pos))?;
//                 *word_pos += 1;
//             }
//             // sub
//             SUB => {
//                 let a = f64::from_bits(
//                     *stack
//                         .get(*stack_pos - 1)
//                         .ok_or(BytecodeRunnerError::StackOverflow(*stack_pos - 1))?
//                         as u64,
//                 );
//                 let b = f64::from_bits(
//                     *stack
//                         .get(*stack_pos - 2)
//                         .ok_or(BytecodeRunnerError::StackOverflow(*stack_pos - 2))?
//                         as u64,
//                 );
//                 let res = b - a;
//                 let slot = stack
//                     .get_mut(*stack_pos - 2)
//                     .ok_or(BytecodeRunnerError::StackOverflow(*stack_pos - 2))?;
//                 *slot = res.to_bits() as usize;
//                 *stack_pos -= 1;

//                 let word_pos = frame_stack
//                     .get_mut(*frame_pos)
//                     .ok_or(BytecodeRunnerError::FrameOverflow(*frame_pos))?;
//                 *word_pos += 1;
//             }
//             // mul
//             MUL => {
//                 let a = f64::from_bits(
//                     *stack
//                         .get(*stack_pos - 1)
//                         .ok_or(BytecodeRunnerError::StackOverflow(*stack_pos - 1))?
//                         as u64,
//                 );
//                 let b = f64::from_bits(
//                     *stack
//                         .get(*stack_pos - 2)
//                         .ok_or(BytecodeRunnerError::StackOverflow(*stack_pos - 2))?
//                         as u64,
//                 );
//                 let res = a * b;
//                 let slot = stack
//                     .get_mut(*stack_pos - 2)
//                     .ok_or(BytecodeRunnerError::StackOverflow(*stack_pos - 2))?;
//                 *slot = res.to_bits() as usize;
//                 *stack_pos -= 1;

//                 let word_pos = frame_stack
//                     .get_mut(*frame_pos)
//                     .ok_or(BytecodeRunnerError::FrameOverflow(*frame_pos))?;
//                 *word_pos += 1;
//             }
//             // div
//             DIV => {
//                 let a = f64::from_bits(
//                     *stack
//                         .get(*stack_pos - 1)
//                         .ok_or(BytecodeRunnerError::StackOverflow(*stack_pos - 1))?
//                         as u64,
//                 );
//                 let b = f64::from_bits(
//                     *stack
//                         .get(*stack_pos - 2)
//                         .ok_or(BytecodeRunnerError::StackOverflow(*stack_pos - 2))?
//                         as u64,
//                 );
//                 let res = b / a;
//                 let slot = stack
//                     .get_mut(*stack_pos - 2)
//                     .ok_or(BytecodeRunnerError::StackOverflow(*stack_pos - 2))?;
//                 *slot = res.to_bits() as usize;
//                 *stack_pos -= 1;

//                 let word_pos = frame_stack
//                     .get_mut(*frame_pos)
//                     .ok_or(BytecodeRunnerError::FrameOverflow(*frame_pos))?;
//                 *word_pos += 1;
//             }
//             // mod
//             MOD => {
//                 let a = f64::from_bits(
//                     *stack
//                         .get(*stack_pos - 1)
//                         .ok_or(BytecodeRunnerError::StackOverflow(*stack_pos - 1))?
//                         as u64,
//                 );
//                 let b = f64::from_bits(
//                     *stack
//                         .get(*stack_pos - 2)
//                         .ok_or(BytecodeRunnerError::StackOverflow(*stack_pos - 2))?
//                         as u64,
//                 );
//                 let res = b % a;
//                 let slot = stack
//                     .get_mut(*stack_pos - 2)
//                     .ok_or(BytecodeRunnerError::StackOverflow(*stack_pos - 2))?;
//                 *slot = res.to_bits() as usize;
//                 *stack_pos -= 1;

//                 let word_pos = frame_stack
//                     .get_mut(*frame_pos)
//                     .ok_or(BytecodeRunnerError::FrameOverflow(*frame_pos))?;
//                 *word_pos += 1;
//             }
//             // load
//             LOAD => {
//                 let bytecode = words
//                     .get(word_pos + 1)
//                     .ok_or(BytecodeRunnerError::OutOfBoundWordAccess(word_pos + 1))?;
//                 let slot = stack
//                     .get_mut(*stack_pos)
//                     .ok_or(BytecodeRunnerError::StackOverflow(*stack_pos))?;
//                 *slot = *bytecode;
//                 *stack_pos += 1;

//                 let word_pos = frame_stack
//                     .get_mut(*frame_pos)
//                     .ok_or(BytecodeRunnerError::FrameOverflow(*frame_pos))?;
//                 *word_pos += 2;
//             }
//             // call
//             CALL => {
//                 let stack_previous_pos = *stack_pos - 1;
//                 let next_word_pos = *stack
//                     .get(*stack_pos - 1)
//                     .ok_or(BytecodeRunnerError::StackOverflow(stack_previous_pos))?;

//                 // moving current word pos to have the next pos when returning
//                 let word_pos = frame_stack
//                     .get_mut(*frame_pos)
//                     .ok_or(BytecodeRunnerError::FrameOverflow(*frame_pos))?;
//                 *word_pos += 1;

//                 // adding frame for the entering the called word
//                 *frame_pos += 1;
//                 let ret_slot = frame_stack
//                     .get_mut(*frame_pos)
//                     .ok_or(BytecodeRunnerError::FrameOverflow(*frame_pos))?;
//                 *ret_slot = next_word_pos;
//             }
//             // ret
//             RET => {
//                 *frame_pos -= 1;
//             }
//             // exit
//             EXIT => {
//                 break;
//             }
//             // label
//             LBL => {
//                 // lbl opcode + id
//                 let word_pos = frame_stack
//                     .get_mut(*frame_pos)
//                     .ok_or(BytecodeRunnerError::FrameOverflow(*frame_pos))?;
//                 *word_pos += 2;
//             }
//             // jmp_lbl
//             JMP_LBL => {
//                 let lbl_id = *stack
//                     .get(*stack_pos - 1)
//                     .ok_or(BytecodeRunnerError::StackOverflow(*stack_pos - 1))?;
//                 let word_pos = frame_stack
//                     .get_mut(*frame_pos)
//                     .ok_or(BytecodeRunnerError::FrameOverflow(*frame_pos))?;
//                 let mut pos = *word_pos;
//                 let mut found = false;
//                 while pos < words.len() {
//                     let opcode = *words
//                         .get(pos)
//                         .ok_or(BytecodeRunnerError::OutOfBoundWordAccess(pos))?;
//                     if opcode == LBL {
//                         let id = *words
//                             .get(pos + 1)
//                             .ok_or(BytecodeRunnerError::OutOfBoundWordAccess(pos + 1))?;
//                         if id == lbl_id {
//                             found = true;
//                             break;
//                         }
//                     }
//                     pos += 2;
//                 }

//                 if found {
//                     *word_pos = pos;
//                 } else {
//                     return Err(BytecodeRunnerError::LabelNotFound(lbl_id).into());
//                 }

//                 *stack_pos -= 1;
//             }
//             _ => {
//                 return Err(BytecodeRunnerError::InvalidOpcode.into());
//             }
//         }
//     }

//     Ok(())
// }
