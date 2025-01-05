#[cfg(test)]
mod tests {
    extern crate test;
    use test::Bencher;

    use crate::{
        bytecode_executor, bytecode_executor_computedgoto_words_safe_usizestack,
        compile_to_bytecode, compile_word, execute_tokens, execute_word_by_index, insert_std,
        tokenize, Token, Value, WordBuilder, WordCode,
    };

    use const_format::concatcp;

    const SOURCE_COMPUTE_ONLY: &str =
        "1 2 + 6 - 10 * 2 / 2 + 6 - 10 * 2 / 6 / 9 * 2 + 6 - 10 * 2 / 2 + 6 - 10 * 2 / 6 / 9 * 1 2 + 6 - 10 * 2 / 2 + 6 - 10 * 2 / 6 / 9 * 2 + 6 - 10 * 2 / 2 + 6 - 10 * 2 / 6 / 9 *";
    const SOURCE: &str = concatcp!(": test ", SOURCE_COMPUTE_ONLY, " ;");
    const SOURCE_AND_CALL: &str = concatcp!(": test ", SOURCE_COMPUTE_ONLY, " ; test");

    const SOURCE_EXP: &str = concatcp!(SOURCE_COMPUTE_ONLY, " exit");

    #[bench]
    fn bench_direct_threaded(b: &mut Bencher) {
        let mut words = crate::Words::with_capacity(64);
        insert_std(&mut words);
        let mut stack: Vec<Value> = Vec::with_capacity(64);

        let toks = tokenize(SOURCE, Some(&mut words)).unwrap();
        let main_word = WordBuilder::new("_repl_main")
            .description("Main REPL function, the one actually interpreted")
            .code(WordCode::Source(toks.to_vec()))
            .build();
        let main_id = words.insert(main_word);

        execute_word_by_index(main_id, &mut words, &mut stack).unwrap();

        let test_word = words.get_index("test").unwrap();
        b.iter(|| {
            execute_word_by_index(test_word, &mut words, &mut stack).unwrap();
            stack.clear();
        });
    }

    #[bench]
    fn bench_bytecode_indirect_threaded(b: &mut Bencher) {
        let mut words = Vec::new();
        let mut ret_stack = Vec::with_capacity(64);
        let mut stack = Vec::with_capacity(64);
        stack.resize(128, 0);
        let toks = tokenize(SOURCE_EXP, None).unwrap();

        let bytecode = compile_to_bytecode(&toks).unwrap();
        words.extend(bytecode);

        ret_stack.push(0);

        let mut stack_pos = 0;
        let mut ret_stack_pos = 0;
        b.iter(|| {
            stack_pos = 0;
            ret_stack_pos = 0;
            let word_pos = ret_stack.get_mut(ret_stack_pos).unwrap();
            *word_pos = 0;
            bytecode_executor_computedgoto_words_safe_usizestack(
                &mut stack,
                &mut stack_pos,
                &mut ret_stack,
                &mut ret_stack_pos,
                &mut words,
            )
            .unwrap();
        });
    }

    #[test]
    fn test_simple_compute() {
        let mut words = crate::Words::new();
        insert_std(&mut words);
        let mut stack = Vec::new();

        let toks = tokenize(SOURCE, Some(&mut words)).unwrap();
        execute_tokens(&toks, &mut words, &mut stack).unwrap();
        assert_eq!(stack.pop().unwrap(), Value::Number(-5523.75));
    }
}
