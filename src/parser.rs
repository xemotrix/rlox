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

    pub fn consume(&mut self, tt: TokenType) {
        if std::mem::discriminant(self.current()) == std::mem::discriminant(&tt) {
            self.advance();
            return;
        }
        println!("Error: Expected {:?}, found {:?}", tt, self.current());
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

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();

        let prefix_rule = parse_rules(self.prev()).prefix;

        if prefix_rule.is_none() {
            println!("Error: Expected expression");
            return;
        }

        let prefix_rule = prefix_rule.unwrap();

        prefix_rule.parse(self);

        while precedence <= parse_rules(self.current()).precedence {
            self.advance();
            let infix_rule = parse_rules(self.prev()).infix.unwrap();
            infix_rule.parse(self)
        }
    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    fn unary(&mut self) {
        let operator_type = self.prev().clone();
        self.expression();
        match operator_type {
            TokenType::Minus => self.ops.push(Op::Negate),
            TokenType::Bang => self.ops.push(Op::Not),
            _ => unimplemented!(),
        }
    }

    fn grouping(&mut self) {
        self.expression();
        self.consume(TokenType::RightParen);
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
            TokenType::EqualEqual => self.ops.push(Op::Equal),
            TokenType::BangEqual => {
                self.ops.push(Op::Equal);
                self.ops.push(Op::Not);
            }
            TokenType::Greater => self.ops.push(Op::Greater),
            TokenType::GreaterEqual => {
                self.ops.push(Op::Less);
                self.ops.push(Op::Not);
            }
            TokenType::Less => self.ops.push(Op::Less),
            TokenType::LessEqual => {
                self.ops.push(Op::Greater);
                self.ops.push(Op::Not);
            }
            _ => panic!("Expected operator, found: {:?}", self.prev()),
        }
    }

    fn literal(&mut self) {
        match self.prev() {
            TokenType::True => self.ops.push(Op::Constant(Value::Bool(true))),
            TokenType::False => self.ops.push(Op::Constant(Value::Bool(false))),
            TokenType::Nil => self.ops.push(Op::Constant(Value::Nil)),
            _ => panic!("Expected literal, found: {:?}", self.prev()),
        }
    }

    fn number(&mut self) {
        if let TokenType::Number(n) = self.prev() {
            self.ops.push(Op::Constant(Value::Number(*n)));
            return;
        }
        panic!("Expected number");
    }

    fn string(&mut self) {
        if let TokenType::String(s) = self.prev() {
            self.ops.push(Op::Constant(Value::String(s.clone())));
            return;
        }
        panic!("Expected string");
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
    Literal,
    Unary,
    Binary,
    Grouping,
    String,
}

impl RuleFunc {
    fn parse(&self, p: &mut Parser) {
        match self {
            Self::Number => Parser::number(p),
            Self::Literal => Parser::literal(p),
            Self::Unary => Parser::unary(p),
            Self::Binary => Parser::binary(p),
            Self::Grouping => Parser::grouping(p),
            Self::String => Parser::string(p),
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
        TokenType::LeftParen => ParseRule {
            prefix: Some(RuleFunc::Grouping),
            infix: None,
            precedence: Precedence::None,
        },
        TokenType::RightParen => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
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
        TokenType::Star 
            | TokenType::Slash => ParseRule {
            prefix: None,
            infix: Some(RuleFunc::Binary),
            precedence: Precedence::Factor,
        },
        TokenType::Number(_) => ParseRule {
            prefix: Some(RuleFunc::Number),
            infix: None,
            precedence: Precedence::None,
        },
        TokenType::False 
            | TokenType::True 
            | TokenType::Nil => ParseRule {
            prefix: Some(RuleFunc::Literal),
            infix: None,
            precedence: Precedence::None,
        },
        TokenType::Bang => ParseRule {
            prefix: Some(RuleFunc::Unary),
            infix: None,
            precedence: Precedence::None,
        },
        TokenType::EqualEqual 
            | TokenType::BangEqual => ParseRule {
            prefix: None,
            infix: Some(RuleFunc::Binary),
            precedence: Precedence::Equality,
        },
        TokenType::Less 
            | TokenType::LessEqual 
            | TokenType::Greater 
            | TokenType::GreaterEqual => {
            ParseRule {
                prefix: None,
                infix: Some(RuleFunc::Binary),
                precedence: Precedence::Comparison,
            }
        }
        TokenType::String(_) => {
            ParseRule {
                prefix: Some(RuleFunc::String),
                infix: None,
                precedence: Precedence::None,
            }
        }
        TokenType::Eof => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },

        _ => unimplemented!(),
    }
}
