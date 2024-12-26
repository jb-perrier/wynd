use crate::{RuntimeError, Token, Value, WordBuilder, WordCode, Words};

pub struct Runtime<'a> {
    pub words: &'a mut Words,
    pub stack: &'a mut Vec<Value>,
    pub ret_stack: &'a mut Vec<(usize, usize)>,
}

pub fn execute(toks: &[Token], words: &mut Words, stack: &mut Vec<Value>) -> anyhow::Result<()> {
    let main_word = WordBuilder::new("_repl_main")
        .code(WordCode::Source(toks.to_vec()))
        .build();
    let main_id = words.insert(main_word);

    let mut ret_stack: Vec<(usize, usize)> = Vec::new();
    ret_stack.push((main_id, 0));

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
