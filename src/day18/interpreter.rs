use crate::day18::token::Token;
use crate::day18::token::Type;
use crate::day18::lexer::Lexer;

/** Based on https://ruslanspivak.com/lsbasi-part1/
*/
#[derive(Debug)]
pub struct Interpreter {
    lexer: Lexer,
    current_token: Option<Token>,
    l2r: bool,
}

impl Interpreter {
    pub fn new(lexer: Lexer, l2r: bool) -> Interpreter {
        let mut interpreter = Interpreter {
            lexer,
            current_token: None,
            l2r,
        };
        interpreter.current_token = interpreter.lexer.get_next_token();
        interpreter
    }

    /**
    term : expr ( (MUL|DIV) expr )* <br />
    expr : factor ( (PLUS|MINUS) factor )* <br />
    factor : INTEGER | LPAREN term RPAREN
    */
    pub fn term(&mut self) -> Option<isize> {
        let expr = match self.l2r {
            true => { Interpreter::factor }
            false => { Interpreter::expr }
        };
        let mut result = self.expr();
        while let Some(token) = self.current_token {
            match token.token_type {
                Type::MUL => {
                    self.eat(Type::MUL);
                    result = Some(result? * expr(self)?);
                }
                Type::DIV => {
                    self.eat(Type::DIV);
                    result = Some(result? / expr(self)?);
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
    term : expr ( (MUL|DIV) expr )* <br />
    expr : factor ( (PLUS|MINUS) factor )* <br />
    factor : INTEGER | LPAREN term RPAREN
    */
    fn expr(&mut self) -> Option<isize> {
        let mut result = self.factor();
        while let Some(token) = self.current_token {
            match token.token_type {
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

    fn factor(&mut self) -> Option<isize> {
        let token = self.current_token?;
        match token.token_type {
            Type::INTEGER => {
                self.eat(Type::INTEGER);
                token.token_value
            }
            Type::LPAREN => {
                self.eat(Type::LPAREN);
                let result = self.term();
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
    use crate::day18::interpreter::Interpreter;
    use crate::day18::lexer::Lexer;

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
        assert_eq!(Some(14), Interpreter::new(Lexer::new("7 * 4 / 2"), true).term());
    }

    #[test]
    fn test_operations_l2r() {
        assert_eq!(Some(71), Interpreter::new(Lexer::new("1 + 2 * 3 + 4 * 5 + 6"), true).term());
    }


    #[test]
    fn test_parenthesis_l2r() {
        assert_eq!(Some(51), Interpreter::new(Lexer::new("1 + (2 * 3) + (4 * (5 + 6))"), true).term());
        assert_eq!(Some(26), Interpreter::new(Lexer::new("2 * 3 + (4 * 5)"), true).term());
        assert_eq!(Some(437), Interpreter::new(Lexer::new("5 + (8 * 3 + 9 + 3 * 4 * 3)"), true).term());
        assert_eq!(Some(12240), Interpreter::new(Lexer::new("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), true).term());
        assert_eq!(Some(13632), Interpreter::new(Lexer::new("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), true).term());
    }

    #[test]
    fn test_day18_part2_examples() {
        assert_eq!(Some(51), Interpreter::new(Lexer::new("1 + (2 * 3) + (4 * (5 + 6))"), false).term());
        assert_eq!(Some(46), Interpreter::new(Lexer::new("2 * 3 + (4 * 5)"), false).term());
        assert_eq!(Some(1445), Interpreter::new(Lexer::new("5 + (8 * 3 + 9 + 3 * 4 * 3)"), false).term());
        assert_eq!(Some(669060), Interpreter::new(Lexer::new("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), false).term());
        assert_eq!(Some(23340), Interpreter::new(Lexer::new("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), false).term());
    }

    #[test]
    fn test_day18_part2() {
        let input = std::fs::read_to_string("resources/day18.txt").unwrap();
        let sum: isize = input
            .lines()
            .map(|line| Interpreter::new(Lexer::new(line), false).term().unwrap())
            .sum();
        println!("{}", sum);
    }

    #[test]
    fn test_day18_part1() {
        let input = std::fs::read_to_string("resources/day18.txt").unwrap();
        let sum: isize = input
            .lines()
            .map(|line| Interpreter::new(Lexer::new(line), true).term().unwrap())
            .sum();
        println!("{}", sum);
    }
}