use crate::interpreter::token::Token;
use crate::interpreter::token::Type;
use std::borrow::Borrow;
use crate::interpreter::lexer::Lexer;

/** Based on https://ruslanspivak.com/lsbasi-part1/
*/
#[derive(Debug)]
struct Interpreter {
    lexer: Lexer,
    current_token: Option<Token>,
}

impl Interpreter {
    fn new(lexer: Lexer) -> Interpreter {
        let mut interpreter = Interpreter {
            lexer,
            current_token: None,
        };
        interpreter.current_token = interpreter.lexer.get_next_token();
        println!("{:?}", interpreter.current_token);
        interpreter
    }


    fn term(&mut self) -> Option<isize> {
        let token = self.current_token?;
        self.eat(Type::INTEGER);
        token.token_value
    }

    /**
    expr : term ( (PLUS|MINUS) term )* <br />
    term : factor ( (MUL|DIV) factor )* <br />
    factor : INTEGER

    */
    fn expr(&mut self) -> Option<isize> {
        println!("{:?}", self);
        let mut result = self.factor();
        while let Some(token) = self.current_token {
            match token.token_type {
                /*Type::INTEGER => {}
                Type::PLUS => {}
                Type::MINUS => {}*/
                Type::MUL => {
                    self.eat(Type::MUL);
                    result = Some(result? * self.factor()?);
                }
                Type::DIV => {
                    self.eat(Type::DIV);
                     result = Some(result? / self.factor()?);
                }
                Type::PLUS => {
                    self.eat(Type::PLUS);
                    result = Some(result? + self.factor()?);
                }
                Type::MINUS => {
                    self.eat(Type::MINUS);
                    result = Some(result? - self.factor()?);
                }
                _ => { break; }
            }
        }
        result
    }

    /**
    expr = factor ( (MUL|DIV) factor)*
    */
    fn factor(&mut self) -> Option<isize>{
        let token = self.current_token;
        match token {
            None => {panic!()}
            Some(token) => {
                self.eat(Type::INTEGER);
                token.token_value
            }
        }
    }


    fn eat(&mut self, expected_type: Type) {
        if self.current_token.is_none() || self.current_token.unwrap().token_type != expected_type {
            panic!("Unexpected token type");
        }
        self.current_token = self.lexer.get_next_token();
    }
}


#[cfg(test)]
mod tests {
    use crate::interpreter::interpreter::Interpreter;
    use crate::interpreter::lexer::Lexer;

    #[test]
    fn test_plus_minus_two_terms() {
        assert_eq!(8, Interpreter::new(Lexer::new("3+5")).expr().unwrap());

        assert_eq!(8, Interpreter::new(Lexer::new("13-5")).expr().unwrap());
    }

    #[test]
    fn test_plus_minus_multiple_terms() {
        assert_eq!(8, Interpreter::new(Lexer::new("1+2+3+2")).expr().unwrap());
        assert_eq!(5, Interpreter::new(Lexer::new("10 + 1 + 2 - 3 + 4 + 6 - 15")).expr().unwrap());
    }

    #[test]
    fn test_mul_div() {
        assert_eq!(14, Interpreter::new(Lexer::new("7 * 4 / 2")).expr().unwrap());
    }

    #[test]
    fn test_operations_l2r() {
        assert_eq!(71, Interpreter::new(Lexer::new("1 + 2 * 3 + 4 * 5 + 6")).expr().unwrap());
    }
}