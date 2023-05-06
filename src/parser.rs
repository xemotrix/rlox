use crate::token::TokenType;
use crate::value::Value;
use crate::Op;

use std::fmt::Debug;

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
    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();

        let prefix_rule = parse_rules(self.prev()).prefix;

        if prefix_rule.is_none() {
            println!("Error: Expected expression");
            return;
        }

        let prefix_rule = prefix_rule.unwrap();

        prefix_rule.func(self);

        while precedence <= parse_rules(self.current()).precedence {
            self.advance();
            let infix_rule = parse_rules(self.prev()).infix.unwrap();
            infix_rule.func(self)
        }

    }
    fn unary(&mut self) {
        todo!()
    }
    fn binary(&mut self) {

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
    }
    fn number(&mut self) {
        if let TokenType::Number(n) = self.prev() {
            self.ops.push(Op::Constant(Value::Number(*n)));
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
