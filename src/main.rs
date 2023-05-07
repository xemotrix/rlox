mod chunk;
mod op;
mod parser;
mod scanner;
mod token;
mod value;
mod vm;
mod symtable;

use chunk::Chunk;
use op::Op;
use scanner::Scanner;
use value::Value;
use vm::VM;

fn main() {
    let line = r#""st" + "ri" + "ng""#;
    println!("Line: '{}'", line);

    let tokens = Scanner::new(line).scan_tokens();

    println!("Tokens:");
    tokens
        .iter()
        .enumerate()
        .for_each(|(i, token)| println!("{:>2} -> {:?}", i, token));

    let mut parser = parser::Parser::new(tokens);
    parser.parse();

    let mut chunk = Chunk::new();
    parser
        .ops
        .into_iter()
        .for_each(|op| chunk.write_chunk(op, 1));

    chunk.write_chunk(Op::Dump, 123);

    println!();
    chunk.dissassemble_chunk("test chunk");

    let mut vm = VM::new();
    vm.chunk = chunk;
    vm.interpret();
}

fn dummy_test() {
    let mut chunk = Chunk::new();
    // chunk.write_chunk(Op::Return, 123);
    chunk.write_chunk(Op::Constant(Value::Number(42.0)), 123);
    chunk.write_chunk(Op::Constant(Value::Number(3.0)), 123);
    chunk.write_chunk(Op::Add, 123);
    chunk.write_chunk(Op::Dump, 123);
    chunk.dissassemble_chunk("test chunk");

    let mut vm = VM::new();
    vm.chunk = chunk;
    vm.interpret();
}
