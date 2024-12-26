use crate::{Runtime, WordBuilder, WordCode};

pub fn insert_help(words: &mut crate::Words) {
    words.insert(
        WordBuilder::new("help")
            .code(WordCode::Native(help))
            .description("Print help")
            .build(),
    );
}

pub fn help(run: &mut Runtime) -> anyhow::Result<()> {
    println!("Type \"words\" to show all available words");
    println!("Type \"quit\" to exit the interpreter");
    println!();
    println!("Github: https://github.com/jb-perrier/wynd");
    println!("Author: Jean-Baptiste Perrier");
    println!("License: MIT");
    Ok(())
}