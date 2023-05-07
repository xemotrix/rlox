use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Bool(bool),
    String(String),
    Nil,
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[test]
fn test_size() {
    println!("size of value: {}", std::mem::size_of::<Value>());
    println!("size of f64: {}", std::mem::size_of::<f64>());
    println!("size of bool: {}", std::mem::size_of::<bool>());
    println!("size of String: {}", std::mem::size_of::<String>());
    println!("size of Value::Number: {}", std::mem::size_of_val(&Value::Number(123.0)));
    println!("size of Value::Bool: {}", std::mem::size_of_val(&Value::Bool(true)));
    println!("size of Value::String: {}", std::mem::size_of_val(&Value::String("hello".to_string())));
    println!("size of Value::Nil: {}", std::mem::size_of_val(&Value::Nil));
    assert_eq!(std::mem::size_of::<Value>(), 32);
}
