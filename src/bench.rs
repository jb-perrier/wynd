#[cfg(test)]
mod tests {
    extern crate test;
    use test::Bencher;

    use crate::{
        compile_to_bytecode, execute_bytecode, execute_bytecode_targetspeed, rust_compute, std::insert_std, tokenize, Frame, Runtime, Value, WordBuilder, Words
    };

    use const_format::concatcp;

    const SOURCE_COMPUTE_ONLY: &str =
        "1 2 + 6 - 10 * 2 / 2 + 6 - 10 * 2 / 6 / 9 * 2 + 6 - 10 * 2 / 2 + 6 - 10 * 2 / 6 / 9 * 1 2 + 6 - 10 * 2 / 2 + 6 - 10 * 2 / 6 / 9 * 2 + 6 - 10 * 2 / 2 + 6 - 10 * 2 / 6 / 9 *";
    const SOURCE: &str = concatcp!(": test ", SOURCE_COMPUTE_ONLY, " ;");
    const SOURCE_AND_CALL: &str = concatcp!(": test ", SOURCE_COMPUTE_ONLY, " ; test");

    const SOURCE_WITH_EXIT: &str = concatcp!(SOURCE_COMPUTE_ONLY, " exit");

    #[bench]
    fn bench_bytecode_arithmetic_target(b: &mut Bencher) {
        let toks = tokenize(SOURCE_WITH_EXIT).unwrap();
        let mut words = Words::new();
        insert_std(&mut words);

        let bytecode = compile_to_bytecode(&toks, &mut words).unwrap();

        let mut values = vec![0; 128];
        let mut frames = vec![0; 128];

        b.iter(|| {
            let mut stack_pos = 0;
            let mut frame_pos = 0;
            frames[0] = 0;
            execute_bytecode_targetspeed(
                &bytecode,
                &mut values,
                &mut stack_pos,
                &mut frames,
                &mut frame_pos,
            )
            .unwrap();
        });
    }

    #[bench]
    fn bench_bytecode_arithmetic_current(b: &mut Bencher) {
        let toks = tokenize(SOURCE_WITH_EXIT).unwrap();
        let mut words = Words::new();
        insert_std(&mut words);

        let bytecode = compile_to_bytecode(&toks, &mut words).unwrap();
        let main_word = WordBuilder::new("main").bytecode(bytecode).build();
        let main_id = words.insert(main_word);

        let mut values = vec![Value::Number(0.0); 128];
        let mut frames = vec![Frame::default(); 128];
        
        let mut runtime = Runtime::new(&mut values, &mut frames, &mut words);
        b.iter(|| {
            runtime.clear();
            runtime.push_frame(Frame {
                word: main_id,
                bytecode: 0,
            });
            execute_bytecode(&mut runtime).unwrap();
        });
    }

    #[test]
    fn test_bytecode_arithmetic() {
        let toks = tokenize(SOURCE_WITH_EXIT).unwrap();
        let mut words = Words::new();
        insert_std(&mut words);

        let bytecode = compile_to_bytecode(&toks, &mut words).unwrap();
        let main_word = WordBuilder::new("main").bytecode(bytecode).build();
        let main_id = words.insert(main_word);

        let mut values = vec![Value::Number(0.0); 128];
        let mut frames = vec![Frame::default(); 128];
        frames[0] = Frame {
            word: main_id,
            bytecode: 0,
        };
        let mut runtime = Runtime::new(&mut values, &mut frames, &mut words);
        execute_bytecode(&mut runtime).unwrap();
        let result = runtime.values()[0].clone();
        assert_eq!(result, Value::Number(-5523.75));
    }

    //     #[bench]
    //     fn bench_bytecode_indirect_threaded(b: &mut Bencher) {
    //         let mut words = Vec::new();
    //         let mut ret_stack = Vec::with_capacity(64);
    //         let mut stack = Vec::with_capacity(64);
    //         stack.resize(128, 0);
    //         let toks = tokenize(SOURCE_EXP, None).unwrap();

    //         let bytecode = compile_to_bytecode(&toks).unwrap();
    //         words.extend(bytecode);

    //         ret_stack.push(0);

    //         let mut stack_pos = 0;
    //         let mut ret_stack_pos = 0;
    //         b.iter(|| {
    //             stack_pos = 0;
    //             ret_stack_pos = 0;
    //             let word_pos = ret_stack.get_mut(ret_stack_pos).unwrap();
    //             *word_pos = 0;
    //             execute_bytecode(
    //                 &mut stack,
    //                 &mut stack_pos,
    //                 &mut ret_stack,
    //                 &mut ret_stack_pos,
    //                 &mut words,
    //             )
    //             .unwrap();
    //         });
    //     }

    #[bench]
    fn bench_rust(b: &mut Bencher) {
        b.iter(|| {
            test::black_box(rust_compute());
        });
    }
}
