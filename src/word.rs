use std::fmt::Display;

use crate::{Value, ValueType, WordCode};

#[derive(Debug)]
pub enum WordForm {
    Postfix,
    Infix,
    Prefix,
    Special
}

impl Display for WordForm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            WordForm::Postfix => "Postfix",
            WordForm::Infix => "Infix",
            WordForm::Prefix => "Prefix",
            WordForm::Special => "Special",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub struct WordParam {
    pub(crate) typ: ValueType,
    pub(crate) description: String,
}

#[derive(Debug)]
pub struct WordAbi {
    pub(crate) form: WordForm,
    pub(crate) input: Vec<WordParam>,
    pub(crate) output: Vec<WordParam>,
}

impl WordAbi {
    pub fn new() -> Self {
        Self {
            form: WordForm::Postfix,
            input: Vec::new(),
            output: Vec::new(),
        }
    }
}

impl std::fmt::Display for WordAbi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let input = self
            .input
            .iter()
            .map(|v| v.typ.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        let output = self
            .output
            .iter()
            .map(|v| v.typ.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        if !input.is_empty() || !output.is_empty() {
            return write!(f, "({}) -> ({})", input, output);
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Word {
    pub(crate) name: String,
    pub(crate) id: usize,
    pub(crate) code: WordCode,
    pub(crate) abi: WordAbi,
    pub(crate) description: Option<String>,
}

impl Word {}

pub struct WordBuilder {
    word: Word,
}

impl WordBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            word: Word {
                name: name.into(),
                id: 0,
                code: WordCode::Source(Vec::new()),
                description: None,
                abi: WordAbi::new(),
            },
        }
    }

    pub fn code(mut self, code: WordCode) -> Self {
        self.word.code = code;
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.word.description = Some(description.into());
        self
    }

    pub fn form(mut self, form: WordForm) -> Self {
        self.word.abi.form = form;
        self
    }

    pub fn input(mut self, typ: ValueType, desc: impl Into<String>) -> Self {
        self.word.abi.input.push(WordParam {
            typ,
            description: desc.into(),
        });
        self
    }

    pub fn output(mut self, typ: ValueType, desc: impl Into<String>) -> Self {
        self.word.abi.output.push(WordParam {
            typ,
            description: desc.into(),
        });
        self
    }

    pub fn build(self) -> Word {
        self.word
    }
}
