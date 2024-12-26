use comfy_table::Table;

use crate::{Runtime, WordBuilder, WordCode, Words};

pub fn insert_word(words: &mut Words) {
    words.insert(WordBuilder::new(":").code(WordCode::Native(define_word)).description("Define a new word").build());
    words.insert(WordBuilder::new("words").code(WordCode::Native(list_words)).description("List all words").build());
}

const HEADER_BOTTOM_ONLY: &str = "   ─ ──       ─    ";

pub fn list_words(run: &mut Runtime) -> anyhow::Result<()> {
    let mut words: Vec<(&str, usize)> = run.words.names().collect();
    words.sort_by(|a, b| a.0.cmp(b.0));

    let mut table = Table::new();
    table.load_preset(HEADER_BOTTOM_ONLY);
    table.set_header(vec!["Word", "Abi", "Description", "Form"]);
    for word in &words {
        let word = run.words.get_by_index(word.1).unwrap();
        let desc = word.description.as_deref().unwrap_or("");
        let abi = word.abi.to_string();
        table.add_row(vec![word.name.as_str(), &abi, desc, word.abi.form.to_string().as_str()]);
    }

    println!("{table}");
    Ok(())
}

pub fn define_word(run: &mut Runtime) -> anyhow::Result<()> {
    let (name, body) = {
        let mut body = Vec::new();
        let (word_id, mut pos) = run.ret_stack.get(run.ret_stack.len() - 2).unwrap();

        // skip ':'
        // let mut pos = pos + 1;

        let current_word = run
            .words
            .get_by_index(*word_id)
            .ok_or(crate::RuntimeError::UnknownWordId { id: *word_id })?;

        let current_word = match &current_word.code {
            WordCode::Source(body) => body,
            _ => return Err(crate::RuntimeError::UnknownWordId { id: *word_id }.into()),
        };

        let name = current_word
            .get(pos)
            .ok_or(crate::RuntimeError::MissingFunctionName)?;
        let name = name
            .as_string()
            .ok_or(crate::RuntimeError::UnexpectedValue {
                expected: "string".to_string(),
                found: name.type_name().to_string(),
            })?;

        pos += 1;
        while let Some(token) = current_word.get(pos) {
            if let crate::Token::Word(word) = token {
                if word == ";" {
                    let pos = run.ret_stack.last_mut().unwrap();

                    // skip ';'
                    pos.1 += 1;
                    break;
                }
            }
            pos += 1;
            body.push(token.clone());
        }
        (name.to_string(), body)
    };
    run.words.insert(WordBuilder::new(&name).code(WordCode::Source(body)).build());
    run.ret_stack.pop();
    Ok(())
}
