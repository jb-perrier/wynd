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
}