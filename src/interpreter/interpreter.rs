use crate::interpreter::token::Token;
use crate::interpreter::token::Type;
use crate::interpreter::lexer::Lexer;

/** Based on https://ruslanspivak.com/lsbasi-part1/
*/
#[derive(Debug)]
struct Interpreter {
    lexer: Lexer,
    current_token: Option<Token>,
    l2r: bool
}

impl Interpreter {
    fn new(lexer: Lexer, l2r: bool) -> Interpreter {
        let mut interpreter = Interpreter {
            lexer,
            current_token: None,
            l2r
        };
        interpreter.current_token = interpreter.lexer.get_next_token();
        interpreter
    }

    /**
    expr : term ( (PLUS|MINUS) term )* <br />
    term : factor ( (MUL|DIV) factor )* <br />
    factor : INTEGER | LPAREN expr RPAREN
    */
    fn term(&mut self) -> Option<isize> {
        let mut result = self.factor();
        while let Some(token) = self.current_token {
            match token.token_type {
                Type::MUL => {
                    self.eat(Type::MUL);
                    result = Some(result? * self.factor()?);
                }
                Type::DIV => {
                    self.eat(Type::DIV);
                    result = Some(result? / self.factor()?);
                }
                _ => { break; }
            }
        }
        result
    }

    /**
    expr : term ( (PLUS|MINUS) term )* <br />
    term : factor ( (MUL|DIV) factor )* <br />
    factor : INTEGER | LPAREN expr RPAREN
    */
    fn expr(&mut self) -> Option<isize> {
        let function: fn(&mut Interpreter) -> Option<isize> = match self.l2r {
            true => { self::Interpreter::factor }
            false => { self::Interpreter::term }
        };
        let mut result = self.term();
        while let Some(token) = self.current_token {
            match token.token_type {
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
                    result = Some(result? + function(self)?);
                }
                Type::MINUS => {
                    self.eat(Type::MINUS);
                    result = Some(result? - function(self)?);
                }
                _ => { break; }
            }
        }
        result
    }

    fn factor(&mut self) -> Option<isize> {
        let token = self.current_token?;
        match token.token_type {
            Type::INTEGER => {
                self.eat(Type::INTEGER);
                token.token_value
            }
            Type::LPAREN => {
                self.eat(Type::LPAREN);
                let result = self.expr();
                self.eat(Type::RPAREN);
                result
            }
            _ => { panic!() }
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
        assert_eq!(Some(8), Interpreter::new(Lexer::new("3+5"), true).expr());
        assert_eq!(Some(8), Interpreter::new(Lexer::new("13-5"), true).expr());
    }

    #[test]
    fn test_plus_minus_multiple_terms() {
        assert_eq!(Some(8), Interpreter::new(Lexer::new("1+2+3+2"), true).expr());
        assert_eq!(Some(5), Interpreter::new(Lexer::new("10 + 1 + 2 - 3 + 4 + 6 - 15"), true).expr());
    }

    #[test]
    fn test_mul_div() {
        assert_eq!(Some(14), Interpreter::new(Lexer::new("7 * 4 / 2"), true).expr());
    }

    #[test]
    fn test_operations_l2r() {
        assert_eq!(Some(71), Interpreter::new(Lexer::new("1 + 2 * 3 + 4 * 5 + 6"), true).expr());
    }

    #[test]
    fn test_operations_proper() {
        assert_eq!(Some(33), Interpreter::new(Lexer::new("1 + 2 * 3 + 4 * 5 + 6"), false).expr());
    }

    #[test]
    fn test_parenthesis_proper() {
        assert_eq!(Some(22), Interpreter::new(Lexer::new("7 + 3 * (10 / (12 / (3 + 1) - 1))"), false).expr());
        assert_eq!(Some(10), Interpreter::new(Lexer::new("7 + 3 * (10 / (12 / (3 + 1) - 1)) / (2 + 3) - 5 - 3 + (8)"), false).expr());
        assert_eq!(Some(12), Interpreter::new(Lexer::new("7 + (((3 + 2)))"), false).expr());
    }

    #[test]
    fn test_parenthesis_l2r() {
        assert_eq!(Some(51), Interpreter::new(Lexer::new("1 + (2 * 3) + (4 * (5 + 6))"), true).expr());
        assert_eq!(Some(26), Interpreter::new(Lexer::new("2 * 3 + (4 * 5)"), true).expr());
        assert_eq!(Some(437), Interpreter::new(Lexer::new("5 + (8 * 3 + 9 + 3 * 4 * 3)"), true).expr());
        assert_eq!(Some(12240), Interpreter::new(Lexer::new("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), true).expr());
        assert_eq!(Some(13632), Interpreter::new(Lexer::new("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), true).expr());
    }
}