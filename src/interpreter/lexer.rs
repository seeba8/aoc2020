use crate::interpreter::token::{Token, Type};

#[derive(Debug)]
pub struct Lexer {
    text: String,
    //current_token: Option<Token>,
    pos: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        let mut lexer = Lexer {
            text: input.to_string(),
            pos: 0,
            current_char: None,
        };
        lexer.current_char = lexer.next();
        lexer
    }

    fn advance(&mut self) {
        self.current_char = self.next();
    }

    fn integer(&mut self) -> Option<isize> {
        let mut result = String::new();
        loop {
            match self.current_char {
                None => { break; }
                Some(chr) => {
                    if !chr.is_numeric() {
                        break;
                    }
                    result += &chr.to_string();
                    self.advance();
                }
            }
        }
        result.parse().ok()
    }

    pub fn get_next_token(&mut self) -> Option<Token> {
        while let Some(current_char) = self.current_char {
            if current_char.is_whitespace() {
                self.advance();
                continue;
            }
            if current_char.is_numeric() {
                return Some(Token {
                    token_type: Type::INTEGER,
                    token_value: self.integer(),
                });
            }
            if current_char == '+' {
                self.advance();
                return Some(Token {
                    token_type: Type::PLUS,
                    token_value: None,
                });
            }
            if current_char == '-' {
                self.advance();
                return Some(Token {
                    token_type: Type::MINUS,
                    token_value: None,
                });
            }
            if current_char == '*'{
                self.advance();
                return Some(Token {
                    token_type: Type::MUL,
                    token_value: None
                });
            }
           if current_char == '/'{
                self.advance();
                return Some(Token {
                    token_type: Type::DIV,
                    token_value: None
                });
            }
            panic!(format!("Illegal character: '{}'", current_char));
        }
        None
    }
}

impl Iterator for Lexer {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.pos += 1;
        self.text.chars().nth(self.pos - 1)
    }
}