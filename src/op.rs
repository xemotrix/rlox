use std::fmt::{Display, Formatter};

use crate::value::Value;

#[derive(Debug)]
pub enum Op {
    Add,
    Constant(Value),
    Divide,
    Dump,
    Equal,
    Greater,
    Less,
    Multiply,
    Negate,
    Nop,
    Not,
    Return,
    Subtract,
}

impl Display for Op {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Op::Add => write!(f, "{:>20} |", "OP_ADD"),
            Op::Constant(v) => write!(f, "{:>20} | {:>4}", "OP_CONSTANT", v),
            Op::Divide => write!(f, "{:>20} |", "OP_DIVIDE"),
            Op::Dump => write!(f, "{:>20} |", "OP_DUMP"),
            Op::Equal => write!(f, "{:>20} |", "OP_EQUAL"),
            Op::Greater => write!(f, "{:>20} |", "OP_GREATER"),
            Op::Less => write!(f, "{:>20} |", "OP_LESS"),
            Op::Multiply => write!(f, "{:>20} |", "OP_MULTIPLY"),
            Op::Negate => write!(f, "{:>20} |", "OP_NEGATE"),
            Op::Nop => write!(f, "{:>20} |", "NOp"),
            Op::Not => write!(f, "{:>20} |", "OP_NOT"),
            Op::Return => write!(f, "{:>20} |", "OP_RETURN"),
            Op::Subtract => write!(f, "{:>20} |", "OP_SUBTRACT"),
        }
    }
}
