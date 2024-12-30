

#[cfg(test)]
mod tests {
    extern crate test;
    use test::Bencher;

    use crate::{execute_tokens, execute_word_by_index, insert_std, tokenize, Value, WordBuilder, WordCode};

    const SOURCE: &str = "1 2 + 6 - 10 * 2 / 2 + 6 - 10 * 2 / 6 / 9 * 2 + 6 - 10 * 2 / 2 + 6 - 10 * 2 / 6 / 9 *";

    #[bench]
    fn bench_simple_compute(b: &mut Bencher) {
        let mut words = crate::Words::with_capacity(64);
        insert_std(&mut words);
        let mut stack: Vec<Value> = Vec::with_capacity(64);

        let toks = tokenize(SOURCE, Some(&mut words)).unwrap();
        let main_word = WordBuilder::new("_repl_main").description("Main REPL function, the one actually interpreted")
        .code(WordCode::Source(toks.to_vec()))
        .build();
        let main_id = words.insert(main_word);

        b.iter(|| {
            test::black_box(execute_word_by_index(main_id, &mut words, &mut stack)).unwrap();
            stack.clear();
        });
    }

    #[test]
    fn test_simple_compute() {
        let mut words = crate::Words::new();
        insert_std(&mut words);
        let mut stack = Vec::new();

        let toks = tokenize(SOURCE, Some(&mut words)).unwrap();
        execute_tokens(&toks, &mut words, &mut stack).unwrap();
        assert_eq!(stack.pop().unwrap(), crate::Value::Number(-5523.75));
    }
}
