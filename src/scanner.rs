use crate::token::TokenType;

pub struct Scanner {
    chars: Vec<char>,
}

struct ScannerIter<'a> {
    scanner: &'a Scanner,
    index: usize,
    stack: Vec<usize>,
}

impl ScannerIter<'_> {
    fn peek(&self) -> Option<&char> {
        self.scanner.chars.get(self.index)
    }
    fn prev(&mut self) {
        self.index -= 1;
    }
    fn save(&mut self) {
        self.stack.push(self.index);
    }
    fn restore(&mut self) {
        self.index = self.stack.pop().unwrap();
    }
}

impl<'a> Iterator for ScannerIter<'a> {
    type Item = &'a char;
    fn next(&mut self) -> Option<Self::Item> {
        self.scanner.chars.get(self.index).map(|c| {
            self.index += 1;
            c
        })
    }
}

impl Scanner {
    pub fn new(str: &str) -> Scanner {
        Scanner {
            chars: str.chars().collect(),
        }
    }

    fn iter(&self) -> ScannerIter {
        ScannerIter {
            scanner: self,
            index: 0,
            stack: Vec::new(),
        }
    }

    pub fn scan_tokens(&self) -> Vec<TokenType> {
        let mut tokens = Vec::new();
        let mut iter_chars = self.iter();

        macro_rules! match_next {
            ($cond:expr => $then:expr; $else:expr) => {
                match iter_chars.peek() {
                    Some($cond) => {
                        iter_chars.next();
                        tokens.push($then);
                    }
                    _ => {
                        tokens.push($else);
                    }
                }
            };
        }

        macro_rules! match_keyword {
            ($chr:expr, $($keyw:expr => $token:expr),* ) => {
                $(
                    if $chr == $keyw.chars().next().unwrap() {
                        iter_chars.save();
                        let maybe_kw = std::iter::once(&$chr)
                            .chain(iter_chars.by_ref().take($keyw.len()-1))
                            .collect::<String>();
                        if maybe_kw == $keyw {
                            tokens.push($token);
                            continue;
                        } else {
                            iter_chars.restore();
                        }
                    }
                )*
            }
        }

        macro_rules! take_while {
            ($chr:expr, $checkfun:expr) => {{
                let s: String = std::iter::once(&$chr)
                    .chain(iter_chars.by_ref().take_while($checkfun))
                    .collect();
                if iter_chars.peek().is_some() {
                    iter_chars.prev();
                }
                s
            }};
        }

        while let Some(c) = iter_chars.next() {
            match c {
                '(' => tokens.push(TokenType::LeftParen),
                ')' => tokens.push(TokenType::RightParen),
                '{' => tokens.push(TokenType::LeftBrace),
                '}' => tokens.push(TokenType::RightBrace),
                ',' => tokens.push(TokenType::Comma),
                '.' => tokens.push(TokenType::Dot),
                '-' => tokens.push(TokenType::Minus),
                '+' => tokens.push(TokenType::Plus),
                ';' => tokens.push(TokenType::Semicolon),
                '/' => {
                    if let Some('/') = iter_chars.peek() {
                        let _: String = iter_chars.by_ref().take_while(|c| **c != '\n').collect();
                    } else {
                        tokens.push(TokenType::Slash);
                    }
                }
                '*' => tokens.push(TokenType::Star),
                '!' => match_next!('=' => TokenType::BangEqual; TokenType::Bang),
                '=' => match_next!('=' => TokenType::EqualEqual; TokenType::Equal),
                '<' => match_next!('=' => TokenType::LessEqual; TokenType::Less),
                '>' => match_next!('=' => TokenType::GreaterEqual; TokenType::Greater),
                '"' => {
                    tokens.push(TokenType::String(
                        take_while!(*c, |chr| **chr != '"')
                            .get(1..)
                            .unwrap()
                            .to_string(),
                    ));
                    iter_chars.next();
                }
                '0'..='9' => {
                    tokens.push(TokenType::Number(
                        take_while!(*c, |chr| chr.is_alphanumeric() || **chr == '_')
                            .parse::<f64>()
                            .unwrap(),
                    ));
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    match_keyword!(*c,
                        "and" => TokenType::And,
                        "class" => TokenType::Class,
                        "else" => TokenType::Else,
                        "false" => TokenType::False,
                        "for" => TokenType::For,
                        "fun" => TokenType::Fun,
                        "if" => TokenType::If,
                        "nil" => TokenType::Nil,
                        "or" => TokenType::Or,
                        "print" => TokenType::Print,
                        "return" => TokenType::Return,
                        "super" => TokenType::Super,
                        "this" => TokenType::This,
                        "true" => TokenType::True,
                        "var" => TokenType::Var,
                        "while" => TokenType::While
                    );

                    tokens.push(TokenType::Identifier(take_while!(*c, |chr| chr
                        .is_alphanumeric()
                        || **chr == '_')));
                }

                ' ' | '\r' | '\t' => {}

                _ => {
                    tokens.push(TokenType::Error);
                    println!("Unexpected character: {}", c);
                }
            }
        }
        tokens.push(TokenType::Eof);
        tokens
    }
}
