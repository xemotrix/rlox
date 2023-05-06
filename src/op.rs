use std::fmt::{Display, Formatter};

use crate::value::Value;

#[derive(Debug)]
pub enum Op {
    Nop,
    Return,
    Constant(Value),
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
    Dump,
}

impl Display for Op {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Op::Nop => write!(f, "{:>20} |", "NOp"),
            Op::Return => write!(f, "{:>20} |", "OP_RETURN"),
            Op::Constant(v) => write!(f, "{:>20} | {:>4}", "OP_CONSTANT", v),
            Op::Negate => write!(f, "{:>20} |", "OP_NEGATE"),
            Op::Add => write!(f, "{:>20} |", "OP_ADD"),
            Op::Subtract => write!(f, "{:>20} |", "OP_SUBTRACT"),
            Op::Multiply => write!(f, "{:>20} |", "OP_MULTIPLY"),
            Op::Divide => write!(f, "{:>20} |", "OP_DIVIDE"),
            Op::Dump => write!(f, "{:>20} |", "OP_DUMP"),
        }
    }
}
