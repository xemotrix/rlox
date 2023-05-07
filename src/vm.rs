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
    CompileError(String),
    RuntimeError(String),
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
        match self.run() {
            InterpretResult::InterpretOk => {}
            InterpretResult::CompileError(e) => {
                println!("Compile error: {}", e);
            }
            InterpretResult::RuntimeError(e) => {
                println!("Runtime error: {}", e);
            }
        }
    }

    fn run(&mut self) -> InterpretResult {
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
                    let m = self.stack.last_mut();
                    match m {
                        Some(Value::Number(v)) => *v = -*v,
                        Some(x) => {
                            return InterpretResult::RuntimeError(format!("can't negate {:?}", x))
                        }
                        None => {
                            return InterpretResult::RuntimeError("nothing to negate".to_string());
                        }
                    }
                }
                Op::Not => {
                    let m = self.stack.last_mut();
                    match m {
                        Some(Value::Bool(v)) => *v = !*v,
                        Some(x) => {
                            return InterpretResult::RuntimeError(format!("can't negate {:?}", x))
                        }
                        None => {
                            return InterpretResult::RuntimeError("nothing to negate".to_string());
                        }
                    }
                }
                Op::Add => {
                    match (&self.stack.pop(), &self.stack.pop()) {
                        (Some(Value::Number(a)), Some(Value::Number(b))) => self.stack.push(Value::Number(b + a)),
                        (Some(Value::String(a)), Some(Value::String(b))) => self.stack.push(Value::String(format!("{}{}", b, a))),
                        (a, b) => return InterpretResult::RuntimeError(
                            format!("tried to perform binop '+' but arguments are invalid: {:?} {:?}", a, b)
                        )
                    };
                }
                Op::Subtract => {
                    match (&self.stack.pop(), &self.stack.pop()) {
                        (Some(Value::Number(a)), Some(Value::Number(b))) => self.stack.push(Value::Number(b - a)),
                        (a, b) => return InterpretResult::RuntimeError(
                            format!("tried to perform binop '-' but arguments are invalid: {:?} {:?}", a, b)
                        )
                    };
                }
                Op::Multiply => {
                    match (&self.stack.pop(), &self.stack.pop()) {
                        (Some(Value::Number(a)), Some(Value::Number(b))) => self.stack.push(Value::Number(b * a)),
                        (a, b) => return InterpretResult::RuntimeError(
                            format!("tried to perform binop '*' but arguments are invalid: {:?} {:?}", a, b)
                        )
                    };
                }
                Op::Divide => {
                    match (&self.stack.pop(), &self.stack.pop()) {
                        (Some(Value::Number(a)), Some(Value::Number(b))) => self.stack.push(Value::Number(b / a)),
                        (a, b) => return InterpretResult::RuntimeError(
                            format!("tried to perform binop '/' but arguments are invalid: {:?} {:?}", a, b)
                        )
                    };
                }

                Op::Greater => {
                    match (&self.stack.pop(), &self.stack.pop()) {
                        (Some(Value::Number(a)), Some(Value::Number(b))) => self.stack.push(Value::Bool(b > a)),
                        (Some(Value::String(a)), Some(Value::String(b))) => self.stack.push(Value::Bool(b > a)),
                        (a, b) => return InterpretResult::RuntimeError(
                            format!("tried to perform binop '>' but arguments are invalid: {:?} {:?}", a, b)
                        )
                    };
                }
                Op::Less => {
                    match (&self.stack.pop(), &self.stack.pop()) {
                        (Some(Value::Number(a)), Some(Value::Number(b))) => self.stack.push(Value::Bool(b < a)),
                        (Some(Value::String(a)), Some(Value::String(b))) => self.stack.push(Value::Bool(b < a)),
                        (a, b) => return InterpretResult::RuntimeError(
                            format!("tried to perform binop '<' but arguments are invalid: {:?} {:?}", a, b)
                        )
                    };
                }
                Op::Equal => {
                    match (&self.stack.pop(), &self.stack.pop()) {
                        (Some(Value::Bool(a)), Some(Value::Bool(b))) => self.stack.push(Value::Bool(b == a)),
                        (Some(Value::Number(a)), Some(Value::Number(b))) => self.stack.push(Value::Bool(b == a)),
                        (Some(Value::String(a)), Some(Value::String(b))) => self.stack.push(Value::Bool(b == a)),
                        // (Some(_), Some(_)) => self.stack.push(Value::Bool(false)),
                        (a, b) => return InterpretResult::RuntimeError(
                            format!("tried to perform binop '==' but arguments are invalid: {:?} {:?}", a, b)
                        )
                    };

                }

                Op::Dump => match self.stack.last() {
                    Some(val) => {
                        println!("{:?}", val);
                    }
                    None => {
                        return InterpretResult::RuntimeError("nothing to dump".to_string());
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
