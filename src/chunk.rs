use crate::op::Op;

pub struct Chunk {
    pub code: Vec<Op>,
    lines: Vec<i32>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            code: Vec::new(),
            lines: Vec::new(),
        }
    }
    pub fn write_chunk(&mut self, op: Op, line: i32) {
        self.code.push(op);
        self.lines.push(line);
    }

    pub fn dissassemble_chunk(&self, name: &str) {
        println!("== {} ==", name);
        for (i, op) in self.code.iter().enumerate() {
            self.dissassemble_instruction(i, op);
        }
    }

    pub fn dissassemble_instruction(&self, offset: usize, op: &Op) {
        println!("{:04} | {:>16}", offset, op);
    }
}
