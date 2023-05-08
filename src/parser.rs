use crate::token::TokenType;
use crate::value::Value;
use crate::Op;

use std::fmt::Debug;

pub struct Parser {
    tokens: Vec<TokenType>,
    prev: usize,
    current: usize,
    scope_depth: i32,
    local_count: usize,
    locals: Vec<Local>,
    pub ops: Vec<Op>,
}

#[derive(Debug)]
struct Local {
    name: String,
    depth: i32,
}

impl Parser {
    pub fn new(tokens: Vec<TokenType>) -> Self {
        Parser {
            tokens,
            prev: 0,
            current: 0,
            scope_depth: 0,
            local_count: 0,
            locals: Vec::new(),
            ops: Vec::new(),
        }
    }

    pub fn compile(&mut self) {
        loop {
            match self.current() {
                TokenType::Eof => break,
                _ => self.declaration(),
            }
        }
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
        let can_assign = precedence <= Precedence::Assignment;
        prefix_rule.parse(self, can_assign);
        while precedence <= parse_rules(self.current()).precedence {
            self.advance();
            let infix_rule = parse_rules(self.prev()).infix.unwrap();
            infix_rule.parse(self, can_assign)
        }

        if can_assign {
            if let TokenType::Equal = self.current() {
                println!("Error: Invalid assignment target.");
            }
        }
    }

    fn declaration(&mut self) {
        match self.current() {
            TokenType::Var => {
                self.advance();
                self.var_declaration();
            },
            _ => {
                self.statement();
            },
        }
    }

    fn var_declaration(&mut self) {
        match self.current() {
            TokenType::Identifier(iden) => {
                let iden_str = iden.clone();
                self.advance();
                self.consume(TokenType::Equal);
                self.expression();
                self.consume(TokenType::Semicolon);

                if self.scope_depth > 0 {
                    self.add_local(iden_str);
                    return;
                }

                self.ops.push(Op::DefineGlobal(iden_str));
            },
            _ => {
                println!("Error: Expected identifier");
            },
        }
    }

    fn add_local(&mut self, iden: String) {
        for local in self.locals.iter().rev() {
            if local.depth < self.scope_depth {
                break;
            }
            if local.name == iden {
                println!("Error: Already variable with this name in this scope.");
            }
        }

        self.local_count += 1;
        self.locals.push(Local {
            name: iden,
            depth: self.scope_depth,
        });
    }

    fn statement(&mut self) {
        match self.current() {
            TokenType::Print => {
                self.advance();
                self.print_statement();
            },
            TokenType::LeftBrace => {
                self.advance();
                self.begin_scope();
                self.block();
                self.end_scope();
            },
            TokenType::If => {
                self.advance();
                self.if_statement();
            }
            _ => {
                self.expression_statement();
            },
        }
    }

    fn if_statement(&mut self) {
        self.consume(TokenType::LeftParen);
        self.expression();
        self.consume(TokenType::RightParen);

        self.ops.push(Op::JumpIfFalse(0));
        let then_jump = self.ops.len() - 1;
        self.ops.push(Op::Pop);


        self.statement();
        

        self.ops.push(Op::Jump(0));
        let else_jump = self.ops.len() - 1;
        self.ops.push(Op::Pop);

        if let Op::JumpIfFalse(ref mut offset) = self.ops[then_jump] {
            *offset = else_jump - then_jump;
        } else { unreachable!() }


        if let TokenType::Else = self.current() {
            self.advance();
            self.statement();
        }

        
        let aux = self.ops.len() - 1 - else_jump;
        if let Op::Jump(ref mut offset) = self.ops[else_jump] {
            *offset = aux;
        } else { unreachable!() }
    }

    fn and(&mut self) {
        self.ops.push(Op::JumpIfFalse(0));
        let jump = self.ops.len() - 1;
        self.ops.push(Op::Pop);

        self.parse_precedence(Precedence::And);

        let off = self.ops.len() - 1 - jump;
        if let Op::JumpIfFalse(ref mut offset) = self.ops[jump] {
            *offset = off;
        } else { unreachable!() }
    }

    fn or(&mut self) {
        self.ops.push(Op::JumpIfTrue(0));
        let jump = self.ops.len() - 1;
        self.ops.push(Op::Pop);

        self.parse_precedence(Precedence::And);

        let off = self.ops.len() - 1 - jump;
        if let Op::JumpIfTrue(ref mut offset) = self.ops[jump] {
            *offset = off;
        } else { unreachable!() }
    }

    fn block(&mut self) {
        loop {
            match self.current() {
                TokenType::RightBrace | TokenType::Eof =>  break,
                _ => self.declaration(),
            }
        }
        self.consume(TokenType::RightBrace);
    }
    fn begin_scope(&mut self) {
        self.scope_depth += 1;
	}
    fn end_scope(&mut self) {
        self.scope_depth -= 1;

        while self.local_count > 0 && self.locals.last().unwrap().depth > self.scope_depth {
            self.local_count -= 1;
            self.locals.pop();
            self.ops.push(Op::Pop);
        }
	}

    fn expression_statement(&mut self) {
        self.expression();
        self.consume(TokenType::Semicolon);
        self.ops.push(Op::Pop);
    }

    fn print_statement(&mut self) {
        self.expression();
        self.consume(TokenType::Semicolon);
        self.ops.push(Op::Print);
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

    fn variable(&mut self, can_assign: bool) {
        self.named_variable(can_assign);
    }

    fn named_variable(&mut self, can_assign: bool) {
        let iden = match self.prev() {
            TokenType::Identifier(iden) =>  iden.clone(),
            _ => panic!("Expected identifier"),
        };

        let (set_op, get_op) = match self.resolve_local(&iden) {
            Some(local) => {
                (Op::SetLocal(local), Op::GetLocal(local))
            },
            None => {
                (Op::SetGlobal(iden.clone()), Op::GetGlobal(iden))
            },
        };

        match self.current() {
            TokenType::Equal if can_assign => {
                self.advance();
                self.expression();
                self.ops.push(set_op);
            },
            _ => {
                self.ops.push(get_op);
            },
        }
    }

    fn resolve_local(&mut self, name: &str) -> Option<usize>{
        self.locals.iter()
            .enumerate()
            .rev()
            .find(|(_, local)| local.name == name)
            .map(|(i, _)| i)
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
    Variable,
    Unary,
    Binary,
    Grouping,
    String,
    And,
    Or,
}

impl RuleFunc {
    fn parse(&self, p: &mut Parser, can_assign: bool) {
        match self {
            Self::Number => Parser::number(p),
            Self::Variable => Parser::variable(p, can_assign),
            Self::Literal => Parser::literal(p),
            Self::Unary => Parser::unary(p),
            Self::Binary => Parser::binary(p),
            Self::Grouping => Parser::grouping(p),
            Self::String => Parser::string(p),
            Self::And => Parser::and(p),
            Self::Or => Parser::or(p),
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
        TokenType::Identifier(_) =>  ParseRule {
            prefix: Some(RuleFunc::Variable),
            infix: None,
            precedence: Precedence::None,
        },
        TokenType::String(_) => ParseRule {
            prefix: Some(RuleFunc::String),
            infix: None,
            precedence: Precedence::None,
        },
        TokenType::And => ParseRule {
            prefix: None,
            infix: Some(RuleFunc::And),
            precedence: Precedence::None,
        },
        TokenType::Or => ParseRule {
            prefix: None,
            infix: Some(RuleFunc::Or),
            precedence: Precedence::None,
        },
        TokenType::Eof | TokenType::Semicolon | TokenType::Equal => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        t => {
            println!("Unknown token: {:?}", t);
            unimplemented!();
        }
    }
}
