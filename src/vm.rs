use crate::chunk::Chunk;
use crate::op::Op;
use crate::value::Value;

pub struct VM {
    pub chunk: Chunk,
    stack: Vec<Value>,
    ip: usize,
}

enum InterpretResult {
    InterpretOk,
    CompileError,
    RuntimeError,
}

impl VM {
    pub fn new() -> VM {
        VM {
            chunk: Chunk::new(),
            ip: 0,
            stack: Vec::new(),
        }
    }
    pub fn interpret(&mut self) {
        self.run();
    }

    fn run(&mut self) -> InterpretResult {
        macro_rules! binOp {
            ($op:tt) => {
                let a = self.stack.pop();
                let b = self.stack.pop();
                match (a, b) {
                    (Some(Value::Number(a)), Some(Value::Number(b))) => {
                        self.stack.push(Value::Number(a $op b));
                    },
                    _ => {
                        return InterpretResult::RuntimeError;
                    }
                }
            }
        }

        loop {
            let op = &self.chunk.code[self.ip];
            match op {
                Op::Nop => {
                    return InterpretResult::InterpretOk;
                }
                Op::Return => {
                    return InterpretResult::InterpretOk;
                }
                Op::Constant(constant) => {
                    self.stack.push(constant.clone());
                }
                Op::Negate => {
                    let m = self.stack.get_mut(0);
                    match m {
                        Some(Value::Number(v)) => *v = -*v,
                        None => {
                            return InterpretResult::RuntimeError;
                        }
                    }
                }
                Op::Add => {
                    binOp!(+);
                }
                Op::Subtract => {
                    binOp!(-);
                }
                Op::Multiply => {
                    binOp!(*);
                }
                Op::Divide => {
                    binOp!(/);
                }
                Op::Dump => match self.stack.last() {
                    Some(val) => {
                        println!("{}", val);
                    }
                    None => {
                        return InterpretResult::RuntimeError;
                    }
                },
            }
            self.ip += 1;
            if self.ip >= self.chunk.code.len() {
                return InterpretResult::InterpretOk;
            }
        }
    }
}
