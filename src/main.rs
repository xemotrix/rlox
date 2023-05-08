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

    // let line = r#"
    //     print 11 + 22*33;
    //     print 420*33;
    //     print "something";
    //     print "something" + " else";
    //     print ("something" == "something else");
    //     var lol = 17 + (42-17);
    //     print lol;
    //     var lmao = "lmao";
    //     var ayy = "WTF";
    //     ayy = "ayy";
    //     print ayy + " " + lmao;
    // "#;

    // let line = r#"
    //     var breakfast = "beignets";
    //     var beverage = "cafe au lait";
    //     breakfast = breakfast + " with " + beverage;

    //     print breakfast;
    // "#;

    // let line = r#"
    // {
    //     var a = "lmao";
    //     {
    //         var a = 1111;
    //         var b = a * 3;
    //         print a;
    //         print b;
    //     }
    //     print a;
    // }
    // "#;

    // let line = r#"
    // {
    //     var a = "lmao";
    //     {
    //         var a = a;
    //         a = "not lmao";
    //         print a;
    //     }
    //     print a;
    // }
    // "#;

    // let line = r#"
    // {
    //     var a = "lm" + "ao";
    //     {
    //         var a = "not " + a;
    //         print a;
    //     }
    //     print a;
    // }
    // "#;

fn main() {
    let line = r#"
        if ( (4==4) and (5==5)) {
            print "true";
        } else {
            print "false";
        }

        print "done";
    "#;
    println!("Line: '{}'", line);

    let tokens = Scanner::new(line).scan_tokens();

    println!("Tokens:");
    tokens
        .iter()
        .enumerate()
        .for_each(|(i, token)| println!("{:>2} -> {:?}", i, token));

    let mut parser = parser::Parser::new(tokens);
    parser.compile();

    let mut chunk = Chunk::new();
    parser
        .ops
        .into_iter()
        .for_each(|op| chunk.write_chunk(op, 1));

    println!();
    chunk.dissassemble_chunk("test chunk");

    let mut vm = VM::new();
    vm.chunk = chunk;
    vm.interpret();
}

