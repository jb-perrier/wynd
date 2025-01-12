use std::fmt::Display;

use crate::{CompiledWordParameters, InterpretedWordParameters, RuntimeWordFn, Token};

#[derive(Debug, Clone, Copy)]
pub enum ValueType {
    Number,
    String,
}

impl ValueType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ValueType::Number => "Number",
            ValueType::String => "String",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum WordImplementation {
    Tokens(Vec<Token>),
    Native {
        interpreted: Option<InterpretedWordFn>,
        compiled: Option<CompiledWordFn>,
    },
}

impl WordImplementation {}


#[derive(Debug)]
pub struct WordParam {
    pub(crate) typ: ValueType,
    pub(crate) description: String,
}

#[derive(Debug)]
pub struct WordAbi {
    pub(crate) input: Vec<WordParam>,
    pub(crate) output: Vec<WordParam>,
}

impl WordAbi {
    pub fn new() -> Self {
        Self {
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

pub type InterpretedWordFn = fn(&mut InterpretedWordParameters) -> Result<(), crate::RuntimeError>;
pub type CompiledWordFn = fn(&mut CompiledWordParameters) -> Result<(), crate::RuntimeError>;

#[derive(Debug)]
pub struct Word {
    name: String,
    abi: WordAbi,
    description: String,
    implementation: WordImplementation,
}

impl Word {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn abi(&self) -> &WordAbi {
        &self.abi
    }

    pub fn implementation(&self) -> &WordImplementation {
        &self.implementation
    }
}

pub struct WordBuilder {
    word: Word,
}

impl WordBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            word: Word {
                name: name.into(),
                description: "".to_string(),
                abi: WordAbi::new(),
                implementation: WordImplementation::Native {
                    interpreted: None,
                    compiled: None,
                },
            },
        }
    }

    pub fn tokens(mut self, tokens: Vec<Token>) -> Self {
        self.word.implementation = WordImplementation::Tokens(tokens);
        self
    }

    pub fn native_interpreted(mut self, func: InterpretedWordFn) -> Self {
        if let WordImplementation::Native { ref mut interpreted, .. } = self.word.implementation {
            *interpreted = Some(func);
        } else {
            self.word.implementation = WordImplementation::Native {
            interpreted: Some(func),
            compiled: None,
            };
        }
        self
    }
    
    pub fn native_compiled(mut self, func: CompiledWordFn) -> Self {
        if let WordImplementation::Native { ref mut compiled, .. } = self.word.implementation {
            *compiled = Some(func);
        } else {
            self.word.implementation = WordImplementation::Native {
            interpreted: None,
            compiled: Some(func),
            };
        }
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.word.description = description.into();
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
