#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
}

impl Value {
    pub fn as_number(&self) -> Option<&f64> {
        match self {
            Value::Number(n) => Some(n),
            _ => None,
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Number(_) => "number",
            Value::String(_) => "string",
        }
    }
}