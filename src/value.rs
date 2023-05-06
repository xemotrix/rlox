use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Value {
    Number(f64),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Value::Number(v) => write!(f, "{}", v),
        }
    }
}

impl Clone for Value {
    fn clone(&self) -> Self {
        match self {
            Value::Number(v) => Value::Number(*v),
        }
    }
}

struct ValueArray {
    v: Vec<Value>,
}
