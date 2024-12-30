use std::f32::consts::E;

use crate::{RuntimeError, Token, Value, WordBuilder, WordCode, Words};

pub struct Runtime<'a> {
    pub words: &'a mut Words,
    pub stack: &'a mut Vec<Value>,
    pub ret_stack: &'a mut Vec<(usize, usize)>,
}

pub fn execute_word_by_index(word_id: usize, words: &mut Words, stack: &mut Vec<Value>) -> anyhow::Result<()> {
    let mut ret_stack: Vec<(usize, usize)> = Vec::new();
    ret_stack.push((word_id, 0));

    while !ret_stack.is_empty() {
        let ret_stack_first_index = ret_stack.len() - 1;
        let (word_id, index) = *ret_stack.last().unwrap();

        let word = words.get_by_index(word_id).unwrap();

        let native_func = match &word.code {
            WordCode::Native(func) => Some(*func),
            WordCode::Source(toks) => {
                if let Some(tok) = toks.get(index) {
                    let pos = ret_stack.get_mut(ret_stack_first_index).unwrap();
                    *pos = (word_id, index + 1);
                    match tok {
                        Token::WordIndex((_, id)) => {
                            ret_stack.push((*id, 0));
                        }
                        Token::Word(word) => {
                            if let Some(id) = words.get_index(word) {
                                ret_stack.push((id, 0));
                            } else {
                                return Err(RuntimeError::UnknownWord { name: word.clone() }.into());
                            }
                        }
                        Token::Number(num) => {
                            stack.push(Value::Number(*num));
                        }
                        Token::String(string) => {
                            stack.push(Value::String(string.clone()));
                        }
                        Token::List(toks) => {
                            let value_list: Result<Vec<Value>, RuntimeError> = toks.iter().map(|tok| match tok {
                                Token::WordIndex(_) => Err(RuntimeError::WordNotAllowedInList),
                                Token::Word(_) => Err(RuntimeError::WordNotAllowedInList),
                                Token::Number(num) => Ok(Value::Number(*num)),
                                Token::String(string) => Ok(Value::String(string.clone())),
                                Token::List(_) => Ok(Value::List(Vec::new())),
                            }).collect();
                            stack.push(Value::List(value_list?));
                        }
                    }
                } else {
                    ret_stack.pop();
                }
                None
            }
        };

        if let Some(func) = native_func {
            let mut runtime = Runtime {
                words,
                stack,
                ret_stack: &mut ret_stack,
            };
            func(&mut runtime)?;
            ret_stack.pop();
        }
    }
    Ok(())
}

pub fn execute_word(name: &str, words: &mut Words, stack: &mut Vec<Value>) -> anyhow::Result<()> {
    let word_id = words.get_index(name).ok_or(RuntimeError::UnknownWord { name: name.to_string() })?;
    execute_word_by_index(word_id, words, stack)
}

pub fn execute_tokens(toks: &[Token], words: &mut Words, stack: &mut Vec<Value>) -> anyhow::Result<()> {
    let main_word = WordBuilder::new("_repl_main").description("Main REPL function, the one actually interpreted")
        .code(WordCode::Source(toks.to_vec()))
        .build();
    let main_id = words.insert(main_word);

    execute_word_by_index(main_id, words, stack)
}
