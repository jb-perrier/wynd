use std::sync::Arc;

#[derive(Debug, Clone, Default)]
pub enum Value {
    #[default]
    Empty,
    Number(f64),
    String(String),
    Any(Arc<dyn std::any::Any + Send + Sync>),
}

impl Value {
    pub fn as_number(&self) -> Option<&f64> {
        match self {
            Value::Number(n) => Some(n),
            _ => None,
        }
    }

    pub fn as_number_mut(&mut self) -> Option<&mut f64> {
        match self {
            Value::Number(n) => Some(n),
            _ => None,
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::Empty => "empty",
            Value::Any(_) => "any",
        }
    }
}