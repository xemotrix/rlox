use crate::token::TokenType;
use crate::value::Value;
use crate::Op;

use std::fmt::{Debug, Formatter};
use std::mem::discriminant;

pub struct Parser {
    tokens: Vec<TokenType>,
    prev: usize,
    current: usize,
    pub ops: Vec<Op>,
}

#[allow(dead_code)]

impl Parser {
    pub fn new(tokens: Vec<TokenType>) -> Self {
        Parser {
            tokens,
            prev: 0,
            current: 0,
            ops: Vec::new(),
        }
    }

    pub fn parse(&mut self) {
        println!("Parsing...");
        self.expression()
    }

    fn current(&self) -> &TokenType {
        &self.tokens[self.current]
    }

    fn prev(&self) -> &TokenType {
        &self.tokens[self.prev]
    }

    fn advance(&mut self) {
        self.prev = self.current;
        
        self.current += 1;

        if self.current == self.tokens.len() {
            println!("We about to panic");
        }
    }

    fn expression(&mut self) {
        println!("Parsing expression...");
        self.parse_precedence(Precedence::Assignment);
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        println!("Parsing precedence '{:?}'...", precedence);
        self.advance();
        if self.current == self.tokens.len() {
            println!("Reached end of tokens");
            return;
        }

        let prefix_rule = parse_rules(self.prev()).prefix;

        if prefix_rule.is_none() {
            println!("Error: Expected expression");
            return;
        }
        let prefix_rule = prefix_rule.unwrap();

        println!("prev: {:?}, prefix rule: {:?}", self.prev(), prefix_rule);
        prefix_rule.func(self);

        println!(
            "while loop: '{:?}' <= '{:?}'",
            precedence,
            parse_rules(self.current()).precedence
        );
        
        if self.current == self.tokens.len() {
            println!("Reached end of tokens");
            return;
        }

        while precedence <= parse_rules(self.current()).precedence {
            self.advance();
            println!("  - advanced to: {:?}", self.current());
            let infix_rule = parse_rules(self.prev()).infix.unwrap();
            println!(
                "  - Prev: '{:?}' -> infix rule: {:?}",
                self.prev(),
                infix_rule
            );
            infix_rule.func(self)
        }

        println!(
            "ended while because precedence '{:?}' > '{:?}'",
            precedence,
            parse_rules(self.current()).precedence
        );
        println!(
            "  current was {:?} with rule -> {:?}",
            self.current(),
            parse_rules(self.current())
        );
    }
    fn unary(&mut self) {
        println!("->unary");
        todo!()
    }
    fn binary(&mut self) {
        println!("->binary");
        println!("  .prev: {:?}", self.prev());
        println!("  .prev rule: {:?}", parse_rules(self.prev()));
        println!("  .prev precedence next: {:?}", parse_rules(self.prev()).precedence.next());

        let operator_type = self.prev().clone();

        let rule = parse_rules(&operator_type);

        self.parse_precedence(rule.precedence.next());

        match operator_type {
            TokenType::Plus => self.ops.push(Op::Add),
            TokenType::Minus => self.ops.push(Op::Subtract),
            TokenType::Star => self.ops.push(Op::Multiply),
            TokenType::Slash => self.ops.push(Op::Divide),
            _ => panic!("Expected operator, found: {:?}", self.prev()),
        }

        println!("\tAdded OP: {:?}", self.ops.last().unwrap());

    }
    fn number(&mut self) {
        println!("->number");
        if let TokenType::Number(n) = self.prev() {
            self.ops.push(Op::Constant(Value::Number(*n)));
            println!("\tAdded OP: {:?}", self.ops.last().unwrap());
            return;
        }
        panic!("Expected number");
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Precedence {
    None,
    Assignment, // =
    Or,         // or
    And,        // and
    Equality,   // == !=
    Comparison, // < > <= >=
    Term,       // + -
    Factor,     // * /
    Unary,      // ! -
    Call,       // . ()
    Primary,
}

impl Precedence {
    fn next(&self) -> Precedence {
        match self {
            Precedence::None => Precedence::Assignment,
            Precedence::Assignment => Precedence::Or,
            Precedence::Or => Precedence::And,
            Precedence::And => Precedence::Equality,
            Precedence::Equality => Precedence::Comparison,
            Precedence::Comparison => Precedence::Term,
            Precedence::Term => Precedence::Factor,
            Precedence::Factor => Precedence::Unary,
            Precedence::Unary => Precedence::Call,
            Precedence::Call => Precedence::Primary,
            Precedence::Primary => Precedence::None,
        }
    }
}

#[derive(Debug)]
enum RuleFunc {
    Number,
    Unary,
    Binary,
}

impl RuleFunc {
    fn func(&self, p: &mut Parser) {
        match self {
            Self::Number => Parser::number(p),
            Self::Unary => Parser::unary(p),
            Self::Binary => Parser::binary(p),
        }
    }
}

#[derive(Debug)]
struct ParseRule {
    prefix: Option<RuleFunc>,
    infix: Option<RuleFunc>,
    precedence: Precedence,
}

fn parse_rules(t: &TokenType) -> ParseRule {
    match t {
        // TokenType::LeftParen => ParseRule {
        //     prefix: Some(|p| p.grouping()),
        //     infix: None,
        //     precedence: Precedence::None,
        // },
        TokenType::Minus => ParseRule {
            prefix: Some(RuleFunc::Unary),
            infix: Some(RuleFunc::Binary),
            precedence: Precedence::Term,
        },
        TokenType::Plus => ParseRule {
            prefix: None,
            infix: Some(RuleFunc::Binary),
            precedence: Precedence::Term,
        },
        TokenType::Star | TokenType::Slash => ParseRule {
            prefix: None,
            infix: Some(RuleFunc::Binary),
            precedence: Precedence::Factor,
        },
        TokenType::Number(_) => ParseRule {
            prefix: Some(RuleFunc::Number),
            infix: None,
            precedence: Precedence::None,
        },
        TokenType::Eof => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        _ => unimplemented!(),
    }
}
