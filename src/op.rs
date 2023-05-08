use std::fmt::{Display, Formatter};

use crate::value::Value;

#[derive(Debug)]
pub enum Op {
    SetGlobal(String),
    GetGlobal(String),
    SetLocal(usize),
    GetLocal(usize),
    DefineGlobal(String),
    Constant(Value),
    Pop,

    Add,
    Subtract,
    Divide,
    Multiply,


    JumpIfFalse(usize),
    JumpIfTrue(usize),
    Jump(usize),

    Equal,
    Greater,
    Less,
    Negate,
    Not,
    Print,
    Nop,
    Return,
}

impl Display for Op {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Op::Add => write!(f, "{:>20} |", "OP_ADD"),
            Op::Constant(v) => write!(f, "{:>20} | {:>4}", "OP_CONSTANT", v),
            Op::SetGlobal(s) => write!(f, "{:>20} | {:?}", "OP_SET_GLOBAL", s),
            Op::GetGlobal(s) => write!(f, "{:>20} | {:?}", "OP_GET_GLOBAL", s),
            Op::SetLocal(s) => write!(f, "{:>20} | {:?}", "OP_SET_LOCAL", s),
            Op::GetLocal(s) => write!(f, "{:>20} | {:?}", "OP_GET_LOCAL", s),
            Op::DefineGlobal(s) => write!(f, "{:>20} | {:?}", "OP_DEFINE_GLOBAL", s),
            Op::JumpIfFalse(i) => write!(f, "{:>20} | {:?}", "OP_JUMP_IF_FALSE", i),
            Op::JumpIfTrue(i) => write!(f, "{:>20} | {:?}", "OP_JUMP_IF_TRUE", i),
            Op::Jump(i) => write!(f, "{:>20} | {:?}", "OP_JUMP", i),
            Op::Divide => write!(f, "{:>20} |", "OP_DIVIDE"),
            Op::Equal => write!(f, "{:>20} |", "OP_EQUAL"),
            Op::Greater => write!(f, "{:>20} |", "OP_GREATER"),
            Op::Less => write!(f, "{:>20} |", "OP_LESS"),
            Op::Multiply => write!(f, "{:>20} |", "OP_MULTIPLY"),
            Op::Negate => write!(f, "{:>20} |", "OP_NEGATE"),
            Op::Nop => write!(f, "{:>20} |", "NOp"),
            Op::Not => write!(f, "{:>20} |", "OP_NOT"),
            Op::Pop => write!(f, "{:>20} |", "OP_POP"),
            Op::Print => write!(f, "{:>20} |", "OP_PRINT"),
            Op::Return => write!(f, "{:>20} |", "OP_RETURN"),
            Op::Subtract => write!(f, "{:>20} |", "OP_SUBTRACT"),
        }
    }
}
