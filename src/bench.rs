// #[cfg(test)]
// mod tests {
//     extern crate test;
//     use test::Bencher;

//     use crate::{
//         compile_to_bytecode, execute_bytecode, insert_std,
//         tokenize,
//     };

//     use const_format::concatcp;

//     const SOURCE_COMPUTE_ONLY: &str =
//         "1 2 + 6 - 10 * 2 / 2 + 6 - 10 * 2 / 6 / 9 * 2 + 6 - 10 * 2 / 2 + 6 - 10 * 2 / 6 / 9 * 1 2 + 6 - 10 * 2 / 2 + 6 - 10 * 2 / 6 / 9 * 2 + 6 - 10 * 2 / 2 + 6 - 10 * 2 / 6 / 9 *";
//     const SOURCE: &str = concatcp!(": test ", SOURCE_COMPUTE_ONLY, " ;");
//     const SOURCE_AND_CALL: &str = concatcp!(": test ", SOURCE_COMPUTE_ONLY, " ; test");

//     const SOURCE_EXP: &str = concatcp!(SOURCE_COMPUTE_ONLY, " exit");

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
// }
