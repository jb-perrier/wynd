use crate::Token;

mod word;
mod errors;
mod module;

pub use module::*;
pub use errors::*;
pub use word::*;

// pub fn parse(tokens: &[Token]) -> Result<ParsingUnit, ParsingError> {
//     let mut inline_tokens = Vec::new();
//     let mut words = Vec::new();
//     let mut i = 0;
//     while i < tokens.len() {
//         match tokens[i] {
//             Token::Word(_) => {
//                 let (word, consumed) = Word::parse(&tokens[i..])?;
//                 words.push(word);
//                 i += consumed;
//             }
//             _ => {
//                 inline_tokens.push(tokens[i].clone());
//                 i += 1;
//             }
//         }
//     }
//     Ok(ParsingUnit { words, inline_tokens })
// }

// pub fn parse_words(tokens: &[Token], mut pos: usize) -> Result<(Word, usize), ParsingError> {
    

//     let name = match tokens[pos] {
//         Token::Word(ref word) => word.clone(),
//         _ => return Err(ParsingError::UnexpectedToken(tokens[pos].clone())),
//     };

//     let builder = WordBuilder::new(name);
//     let mut tokens = Vec::new();

//     pos += 1;
//     while pos < tokens.len() {
//         if let Token::Word(ref word) = tokens[pos] {
//             if word == ";" {
//                 break;
//             }
//         }
//         tokens.push(tokens[pos].clone());
//         pos += 1;
//     }

//     Ok((tokens, pos))
// }
