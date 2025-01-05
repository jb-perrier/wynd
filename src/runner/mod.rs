// use std::f32::consts::E;

// use crate::ExecutorError;

// pub fn execute_bytecode(
//     stack: &mut [usize],
//     stack_pos: &mut usize,
//     frame_stack: &mut [usize],
//     frame_pos: &mut usize,
//     words: &mut [usize],
// ) -> anyhow::Result<()> {
//     /*
//      * USE Option::ok_or_else to avoid the error being evaluated when the Option is Some
//      */
//     loop {
//         let word_pos = *frame_stack
//             .get(*frame_pos)
//             .ok_or(ExecutorError::FrameOverflow(*frame_pos))?;
//         let opcode = *words
//             .get(word_pos)
//             .ok_or(ExecutorError::OutOfBoundWordAccess(word_pos))?;

//         match opcode {
//             // add
//             ADD => {
//                 let a = f64::from_bits(
//                     *stack
//                         .get(*stack_pos - 1)
//                         .ok_or(ExecutorError::StackOverflow(*stack_pos - 1))?
//                         as u64,
//                 );
//                 let b = f64::from_bits(
//                     *stack
//                         .get(*stack_pos - 2)
//                         .ok_or(ExecutorError::StackOverflow(*stack_pos - 2))?
//                         as u64,
//                 );
//                 let res = a + b;
//                 let slot = stack
//                     .get_mut(*stack_pos - 2)
//                     .ok_or(ExecutorError::StackOverflow(*stack_pos - 2))?;
//                 *slot = res.to_bits() as usize;
//                 *stack_pos -= 1;

//                 let word_pos = frame_stack
//                     .get_mut(*frame_pos)
//                     .ok_or(ExecutorError::FrameOverflow(*frame_pos))?;
//                 *word_pos += 1;
//             }
//             // sub
//             SUB => {
//                 let a = f64::from_bits(
//                     *stack
//                         .get(*stack_pos - 1)
//                         .ok_or(ExecutorError::StackOverflow(*stack_pos - 1))?
//                         as u64,
//                 );
//                 let b = f64::from_bits(
//                     *stack
//                         .get(*stack_pos - 2)
//                         .ok_or(ExecutorError::StackOverflow(*stack_pos - 2))?
//                         as u64,
//                 );
//                 let res = b - a;
//                 let slot = stack
//                     .get_mut(*stack_pos - 2)
//                     .ok_or(ExecutorError::StackOverflow(*stack_pos - 2))?;
//                 *slot = res.to_bits() as usize;
//                 *stack_pos -= 1;

//                 let word_pos = frame_stack
//                     .get_mut(*frame_pos)
//                     .ok_or(ExecutorError::FrameOverflow(*frame_pos))?;
//                 *word_pos += 1;
//             }
//             // mul
//             MUL => {
//                 let a = f64::from_bits(
//                     *stack
//                         .get(*stack_pos - 1)
//                         .ok_or(ExecutorError::StackOverflow(*stack_pos - 1))?
//                         as u64,
//                 );
//                 let b = f64::from_bits(
//                     *stack
//                         .get(*stack_pos - 2)
//                         .ok_or(ExecutorError::StackOverflow(*stack_pos - 2))?
//                         as u64,
//                 );
//                 let res = a * b;
//                 let slot = stack
//                     .get_mut(*stack_pos - 2)
//                     .ok_or(ExecutorError::StackOverflow(*stack_pos - 2))?;
//                 *slot = res.to_bits() as usize;
//                 *stack_pos -= 1;

//                 let word_pos = frame_stack
//                     .get_mut(*frame_pos)
//                     .ok_or(ExecutorError::FrameOverflow(*frame_pos))?;
//                 *word_pos += 1;
//             }
//             // div
//             DIV => {
//                 let a = f64::from_bits(
//                     *stack
//                         .get(*stack_pos - 1)
//                         .ok_or(ExecutorError::StackOverflow(*stack_pos - 1))?
//                         as u64,
//                 );
//                 let b = f64::from_bits(
//                     *stack
//                         .get(*stack_pos - 2)
//                         .ok_or(ExecutorError::StackOverflow(*stack_pos - 2))?
//                         as u64,
//                 );
//                 let res = b / a;
//                 let slot = stack
//                     .get_mut(*stack_pos - 2)
//                     .ok_or(ExecutorError::StackOverflow(*stack_pos - 2))?;
//                 *slot = res.to_bits() as usize;
//                 *stack_pos -= 1;

//                 let word_pos = frame_stack
//                     .get_mut(*frame_pos)
//                     .ok_or(ExecutorError::FrameOverflow(*frame_pos))?;
//                 *word_pos += 1;
//             }
//             // mod
//             MOD => {
//                 let a = f64::from_bits(
//                     *stack
//                         .get(*stack_pos - 1)
//                         .ok_or(ExecutorError::StackOverflow(*stack_pos - 1))?
//                         as u64,
//                 );
//                 let b = f64::from_bits(
//                     *stack
//                         .get(*stack_pos - 2)
//                         .ok_or(ExecutorError::StackOverflow(*stack_pos - 2))?
//                         as u64,
//                 );
//                 let res = b % a;
//                 let slot = stack
//                     .get_mut(*stack_pos - 2)
//                     .ok_or(ExecutorError::StackOverflow(*stack_pos - 2))?;
//                 *slot = res.to_bits() as usize;
//                 *stack_pos -= 1;

//                 let word_pos = frame_stack
//                     .get_mut(*frame_pos)
//                     .ok_or(ExecutorError::FrameOverflow(*frame_pos))?;
//                 *word_pos += 1;
//             }
//             // load
//             LOAD => {
//                 let bytecode = words
//                     .get(word_pos + 1)
//                     .ok_or(ExecutorError::OutOfBoundWordAccess(word_pos + 1))?;
//                 let slot = stack
//                     .get_mut(*stack_pos)
//                     .ok_or(ExecutorError::StackOverflow(*stack_pos))?;
//                 *slot = *bytecode;
//                 *stack_pos += 1;

//                 let word_pos = frame_stack
//                     .get_mut(*frame_pos)
//                     .ok_or(ExecutorError::FrameOverflow(*frame_pos))?;
//                 *word_pos += 2;
//             }
//             // call
//             CALL => {
//                 let stack_previous_pos = *stack_pos - 1;
//                 let next_word_pos = *stack
//                     .get(*stack_pos - 1)
//                     .ok_or(ExecutorError::StackOverflow(stack_previous_pos))?;

//                 // moving current word pos to have the next pos when returning
//                 let word_pos = frame_stack
//                     .get_mut(*frame_pos)
//                     .ok_or(ExecutorError::FrameOverflow(*frame_pos))?;
//                 *word_pos += 1;

//                 // adding frame for the entering the called word
//                 *frame_pos += 1;
//                 let ret_slot = frame_stack
//                     .get_mut(*frame_pos)
//                     .ok_or(ExecutorError::FrameOverflow(*frame_pos))?;
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
//                     .ok_or(ExecutorError::FrameOverflow(*frame_pos))?;
//                 *word_pos += 2;
//             }
//             // jmp_lbl
//             JMP_LBL => {
//                 let lbl_id = *stack
//                     .get(*stack_pos - 1)
//                     .ok_or(ExecutorError::StackOverflow(*stack_pos - 1))?;
//                 let word_pos = frame_stack
//                     .get_mut(*frame_pos)
//                     .ok_or(ExecutorError::FrameOverflow(*frame_pos))?;
//                 let mut pos = *word_pos;
//                 let mut found = false;
//                 while pos < words.len() {
//                     let opcode = *words
//                         .get(pos)
//                         .ok_or(ExecutorError::OutOfBoundWordAccess(pos))?;
//                     if opcode == LBL {
//                         let id = *words
//                             .get(pos + 1)
//                             .ok_or(ExecutorError::OutOfBoundWordAccess(pos + 1))?;
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
//                     return Err(ExecutorError::LabelNotFound(lbl_id).into());
//                 }

//                 *stack_pos -= 1;
//             }
//             _ => {
//                 return Err(ExecutorError::InvalidOpcode.into());
//             }
//         }
//     }

//     Ok(())
// }

// #[derive(Debug, Error)]
// pub enum ExecutorError {
//     #[error("Invalid opcode")]
//     InvalidOpcode,
//     #[error("Stack under/over flow, index: {0}")]
//     StackOverflow(usize),
//     #[error("Frame under/over flow, frame index: {0}")]
//     FrameOverflow(usize),
//     #[error("Out of bound word access, index: {0}")]
//     OutOfBoundWordAccess(usize),
//     #[error("Label not found, id: {0}")]
//     LabelNotFound(usize),
// }