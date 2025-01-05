use anyhow::anyhow;
use instructions::{ADD, CALL, DIV, EXIT, JMP_LBL, LBL, LOAD, MOD, MUL, RET, SUB};
use thiserror::Error;

use crate::{word, NativeWordFn, Token, Value, WordCode, Words};

pub fn compile_word(toks: &[Token]) -> anyhow::Result<Vec<usize>> {
    let mut bytecode = Vec::new();

    for tok in toks {
        match tok {
            Token::Word(name) => match name.as_str() {
                "+" => bytecode.push(add as usize),
                "-" => bytecode.push(sub as usize),
                "*" => bytecode.push(mul as usize),
                "/" => bytecode.push(div as usize),
                "%" => bytecode.push(rem as usize),
                _ => todo!(),
            },
            Token::Number(num) => {
                bytecode.push(load_f64 as usize);
                bytecode.push(num.to_bits() as usize);
            }
            _ => todo!(),
        }
    }

    Ok(bytecode)
}

pub fn add(stack: &mut [f64], stack_pos: &mut usize, bytecode: &[usize], pos: &mut usize) {
    unsafe {
        let a = stack.get_unchecked(*stack_pos - 1);
        let b = stack.get_unchecked(*stack_pos - 2);
        // if let (Value::Number(a), Value::Number(b)) = (a, b) {
        // let res = Value::Number(a + b);
        let res = a + b;
        let slot = stack.get_unchecked_mut(*stack_pos - 2);
        *slot = res;
        // }
        *stack_pos -= 1;
    }
}

pub fn sub(stack: &mut [f64], stack_pos: &mut usize, bytecode: &[usize], pos: &mut usize) {
    unsafe {
        let a = stack.get_unchecked(*stack_pos - 1);
        let b = stack.get_unchecked(*stack_pos - 2);
        // if let (Value::Number(a), Value::Number(b)) = (a, b) {
        let res = b - a;
        let slot = stack.get_unchecked_mut(*stack_pos - 2);
        *slot = res;
        // }
        *stack_pos -= 1;
    }
}

pub fn mul(stack: &mut [f64], stack_pos: &mut usize, bytecode: &[usize], pos: &mut usize) {
    unsafe {
        let a = stack.get_unchecked(*stack_pos - 1);
        let b = stack.get_unchecked(*stack_pos - 2);
        // if let (Value::Number(a), Value::Number(b)) = (a, b) {
        let res = a * b;
        let slot = stack.get_unchecked_mut(*stack_pos - 2);
        *slot = res;
        // }
        *stack_pos -= 1;
    }
}

pub fn div(stack: &mut [f64], stack_pos: &mut usize, bytecode: &[usize], pos: &mut usize) {
    unsafe {
        let a = stack.get_unchecked(*stack_pos - 1);
        let b = stack.get_unchecked(*stack_pos - 2);
        // if let (Value::Number(a), Value::Number(b)) = (a, b) {
        let res = b / a;
        let slot = stack.get_unchecked_mut(*stack_pos - 2);
        *slot = res;
        // }
        *stack_pos -= 1;
    }
}

pub fn rem(stack: &mut [f64], stack_pos: &mut usize, bytecode: &[usize], pos: &mut usize) {
    unsafe {
        let a = stack.get_unchecked(*stack_pos - 1);

        let b = stack.get_unchecked(*stack_pos - 2);
        // if let (Value::Number(a), Value::Number(b)) = (a, b) {
        // let res = Value::Number(b % a);
        let res = b % a;
        let slot = stack.get_unchecked_mut(*stack_pos - 2);
        *slot = res;
        // }
        *stack_pos -= 1;
    }
}

pub fn load_f64(stack: &mut [f64], stack_pos: &mut usize, bytecode: &[usize], pos: &mut usize) {
    *pos += 1;
    let num = f64::from_bits(bytecode[*pos] as u64);
    let slot = unsafe { stack.get_unchecked_mut(*stack_pos) };
    *slot = num;
    *stack_pos += 1;
}

type CompiledFn = fn(&mut [f64], &mut usize, &[usize], &mut usize);

pub fn bytecode_executor(
    bytecode: &[usize],
    stack: &mut [f64],
    stack_pos: &mut usize,
) -> anyhow::Result<()> {
    let mut pos = 0;

    while pos < bytecode.len() {
        let func: CompiledFn = unsafe { std::mem::transmute(bytecode[pos]) };
        func(stack, stack_pos, bytecode, &mut pos);
        pos += 1;
    }

    Ok(())
}

pub fn compile_to_bytecode(toks: &[Token]) -> anyhow::Result<Vec<usize>> {
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
                name => {}
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

pub fn bytecode_executor_computedgoto_words_safe_usizestack(
    stack: &mut [usize],
    stack_pos: &mut usize,
    frame_stack: &mut [usize],
    frame_pos: &mut usize,
    words: &mut [usize],
) -> anyhow::Result<()> {
    /*
     * USE Option::ok_or_else to avoid the error being evaluated when the Option is Some
     */
    loop {
        let word_pos = *frame_stack
            .get(*frame_pos)
            .ok_or(ExecutorError::FrameOverflow(*frame_pos))?;
        let opcode = *words
            .get(word_pos)
            .ok_or(ExecutorError::OutOfBoundWordAccess(word_pos))?;

        match opcode {
            // add
            ADD => {
                let a = f64::from_bits(
                    *stack
                        .get(*stack_pos - 1)
                        .ok_or(ExecutorError::StackOverflow(*stack_pos - 1))?
                        as u64,
                );
                let b = f64::from_bits(
                    *stack
                        .get(*stack_pos - 2)
                        .ok_or(ExecutorError::StackOverflow(*stack_pos - 2))?
                        as u64,
                );
                let res = a + b;
                let slot = stack
                    .get_mut(*stack_pos - 2)
                    .ok_or(ExecutorError::StackOverflow(*stack_pos - 2))?;
                *slot = res.to_bits() as usize;
                *stack_pos -= 1;

                let word_pos = frame_stack
                    .get_mut(*frame_pos)
                    .ok_or(ExecutorError::FrameOverflow(*frame_pos))?;
                *word_pos += 1;
            }
            // sub
            SUB => {
                let a = f64::from_bits(
                    *stack
                        .get(*stack_pos - 1)
                        .ok_or(ExecutorError::StackOverflow(*stack_pos - 1))?
                        as u64,
                );
                let b = f64::from_bits(
                    *stack
                        .get(*stack_pos - 2)
                        .ok_or(ExecutorError::StackOverflow(*stack_pos - 2))?
                        as u64,
                );
                let res = b - a;
                let slot = stack
                    .get_mut(*stack_pos - 2)
                    .ok_or(ExecutorError::StackOverflow(*stack_pos - 2))?;
                *slot = res.to_bits() as usize;
                *stack_pos -= 1;

                let word_pos = frame_stack
                    .get_mut(*frame_pos)
                    .ok_or(ExecutorError::FrameOverflow(*frame_pos))?;
                *word_pos += 1;
            }
            // mul
            MUL => {
                let a = f64::from_bits(
                    *stack
                        .get(*stack_pos - 1)
                        .ok_or(ExecutorError::StackOverflow(*stack_pos - 1))?
                        as u64,
                );
                let b = f64::from_bits(
                    *stack
                        .get(*stack_pos - 2)
                        .ok_or(ExecutorError::StackOverflow(*stack_pos - 2))?
                        as u64,
                );
                let res = a * b;
                let slot = stack
                    .get_mut(*stack_pos - 2)
                    .ok_or(ExecutorError::StackOverflow(*stack_pos - 2))?;
                *slot = res.to_bits() as usize;
                *stack_pos -= 1;

                let word_pos = frame_stack
                    .get_mut(*frame_pos)
                    .ok_or(ExecutorError::FrameOverflow(*frame_pos))?;
                *word_pos += 1;
            }
            // div
            DIV => {
                let a = f64::from_bits(
                    *stack
                        .get(*stack_pos - 1)
                        .ok_or(ExecutorError::StackOverflow(*stack_pos - 1))?
                        as u64,
                );
                let b = f64::from_bits(
                    *stack
                        .get(*stack_pos - 2)
                        .ok_or(ExecutorError::StackOverflow(*stack_pos - 2))?
                        as u64,
                );
                let res = b / a;
                let slot = stack
                    .get_mut(*stack_pos - 2)
                    .ok_or(ExecutorError::StackOverflow(*stack_pos - 2))?;
                *slot = res.to_bits() as usize;
                *stack_pos -= 1;

                let word_pos = frame_stack
                    .get_mut(*frame_pos)
                    .ok_or(ExecutorError::FrameOverflow(*frame_pos))?;
                *word_pos += 1;
            }
            // mod
            MOD => {
                let a = f64::from_bits(
                    *stack
                        .get(*stack_pos - 1)
                        .ok_or(ExecutorError::StackOverflow(*stack_pos - 1))?
                        as u64,
                );
                let b = f64::from_bits(
                    *stack
                        .get(*stack_pos - 2)
                        .ok_or(ExecutorError::StackOverflow(*stack_pos - 2))?
                        as u64,
                );
                let res = b % a;
                let slot = stack
                    .get_mut(*stack_pos - 2)
                    .ok_or(ExecutorError::StackOverflow(*stack_pos - 2))?;
                *slot = res.to_bits() as usize;
                *stack_pos -= 1;

                let word_pos = frame_stack
                    .get_mut(*frame_pos)
                    .ok_or(ExecutorError::FrameOverflow(*frame_pos))?;
                *word_pos += 1;
            }
            // load
            LOAD => {
                let bytecode = words
                    .get(word_pos + 1)
                    .ok_or(ExecutorError::OutOfBoundWordAccess(word_pos + 1))?;
                let slot = stack
                    .get_mut(*stack_pos)
                    .ok_or(ExecutorError::StackOverflow(*stack_pos))?;
                *slot = *bytecode;
                *stack_pos += 1;

                let word_pos = frame_stack
                    .get_mut(*frame_pos)
                    .ok_or(ExecutorError::FrameOverflow(*frame_pos))?;
                *word_pos += 2;
            }
            // call
            CALL => {
                let stack_previous_pos = *stack_pos - 1;
                let next_word_pos = *stack
                    .get(*stack_pos - 1)
                    .ok_or(ExecutorError::StackOverflow(stack_previous_pos))?;

                // moving current word pos to have the next pos when returning
                let word_pos = frame_stack
                    .get_mut(*frame_pos)
                    .ok_or(ExecutorError::FrameOverflow(*frame_pos))?;
                *word_pos += 1;

                // adding frame for the entering the called word
                *frame_pos += 1;
                let ret_slot = frame_stack
                    .get_mut(*frame_pos)
                    .ok_or(ExecutorError::FrameOverflow(*frame_pos))?;
                *ret_slot = next_word_pos;
            }
            // ret
            RET => {
                *frame_pos -= 1;
            }
            // exit
            EXIT => {
                break;
            }
            // label
            LBL => {
                // lbl opcode + id
                let word_pos = frame_stack
                    .get_mut(*frame_pos)
                    .ok_or(ExecutorError::FrameOverflow(*frame_pos))?;
                *word_pos += 2;
            }
            // jmp_lbl
            JMP_LBL => {
                let lbl_id = *stack
                    .get(*stack_pos - 1)
                    .ok_or(ExecutorError::StackOverflow(*stack_pos - 1))?;
                let word_pos = frame_stack
                    .get_mut(*frame_pos)
                    .ok_or(ExecutorError::FrameOverflow(*frame_pos))?;
                let mut pos = *word_pos;
                let mut found = false;
                while pos < words.len() {
                    let opcode = *words
                        .get(pos)
                        .ok_or(ExecutorError::OutOfBoundWordAccess(pos))?;
                    if opcode == LBL {
                        let id = *words
                            .get(pos + 1)
                            .ok_or(ExecutorError::OutOfBoundWordAccess(pos + 1))?;
                        if id == lbl_id {
                            found = true;
                            break;
                        }
                    }
                    pos += 2;
                }

                if found {
                    *word_pos = pos;
                } else {
                    return Err(ExecutorError::LabelNotFound(lbl_id).into());
                }

                *stack_pos -= 1;
            }
            _ => {
                return Err(ExecutorError::InvalidOpcode.into());
            }
        }
    }

    Ok(())
}

#[derive(Debug, Error)]
pub enum ExecutorError {
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

mod instructions {
    pub const ADD: usize = 0;
    pub const SUB: usize = 1;
    pub const MUL: usize = 2;
    pub const DIV: usize = 3;
    pub const MOD: usize = 4;
    pub const LOAD: usize = 5;
    pub const CALL: usize = 6;
    pub const RET: usize = 7;
    pub const EXIT: usize = 8;
    pub const LBL: usize = 9;
    pub const JMP_LBL: usize = 10;
}
