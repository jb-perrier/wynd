use crate::{Runtime, RuntimeError, Value, ValueType, WordBuilder, WordCode, Words};

pub fn insert_shell(words: &mut Words) {
    words.insert(WordBuilder::new("ls").code(WordCode::Native(ls)).input(ValueType::String, "Path").description("List directory contents").build());
    words.insert(WordBuilder::new("pwd").code(WordCode::Native(pwd)).description("Print working directory").build());
}

pub fn ls(run: &mut Runtime) -> anyhow::Result<()> {
    let path = run.stack.pop().ok_or(RuntimeError::StackUnderflow())?;
    let path = path.as_string().ok_or(RuntimeError::UnexpectedValue { expected: "string".to_string(), found: path.value_type().as_str().to_string() })?;
    let path = std::path::Path::new(path);
    let entries = std::fs::read_dir(path)?;
    for entry in entries {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let path = entry.path();
        let path = path.to_str().unwrap();
        let file_type = if file_type.is_dir() {
            "Dir"
        } else {
            "File"
        };
        println!("{:<5} {}", file_type, path);
    }
    Ok(())
}

pub fn pwd(run: &mut Runtime) -> anyhow::Result<()> {
    let path = std::env::current_dir()?;
    let path = path.to_str().unwrap();
    run.stack.push(Value::String(path.to_string()));
    Ok(())
}