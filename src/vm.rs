use crate::chunk::Chunk;
use crate::op::Op;
use crate::value::Value;
use crate::symtable::SymTable;

pub struct VM {
    pub chunk: Chunk,
    stack: Vec<Value>,
    symtable: SymTable,
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
            stack: Vec::new(),
            symtable: SymTable::new(),
            ip: 0,
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
        println!("\nRunning...");
        loop {
            let op = &self.chunk.code[self.ip];
            match op {
                Op::Nop => {
                    return InterpretResult::InterpretOk;
                }
                Op::Return => {
                    return InterpretResult::InterpretOk;
                }
                Op::JumpIfFalse(offset) => {
                    if let Value::Bool(false) = self.stack.last().expect("stack is empty") {
                        self.ip += offset;
                    }
                }
                Op::Jump(offset) => {
                    self.ip += offset;
                }
                Op::GetGlobal(iden_str) => {
                    match self.symtable.get(iden_str.clone()) {
                        Some(v) => self.stack.push(v),
                        None => return InterpretResult::RuntimeError(format!("undefined variable '{}'", iden_str)),
                    }
                }
                Op::SetGlobal(iden_str) => {
                    let iden = iden_str.clone();
                    let overwrited = self.symtable.set(
                        iden.clone(), 
                        self.stack.last().expect("stack is empty").clone()
                    );
                    if !overwrited {
                        self.symtable.delete(iden_str).expect("can't delete");
                        return InterpretResult::RuntimeError(format!("undefined variable '{}'", iden_str));
                    }
                }
                Op::SetLocal(idx) => {
                    self.stack[*idx] = self.stack.last().expect("stack is empty").clone();
                }
                Op::GetLocal(idx) => {
                    self.stack.push(self.stack[*idx].clone());
                }
                Op::DefineGlobal(iden_str) => {
                    self.symtable.set(iden_str.clone(), self.stack.pop().expect("stack is empty"));
                }
                Op::Pop => {
                    self.stack.pop();
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

                Op::Print => match self.stack.last() {
                    Some(_) => {
                        println!("{}", self.stack.pop().unwrap());
                    }
                    None => {
                        return InterpretResult::RuntimeError("nothing to dump".to_string());
                    }
                },
            }
            // println!("inst {:?} stack: {:?}", op, self.stack);
            self.ip += 1;
            if self.ip >= self.chunk.code.len() {
                return InterpretResult::InterpretOk;
            }
        }
    }
}
