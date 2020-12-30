#[derive(Debug)]
pub enum Operator {
    ADD,
    MULTIPLY,
}

#[derive(Debug)]
pub enum Value {
    VALUE(isize),
    EXPRESSION(Box<Expression>),
}

#[derive(Debug)]
pub struct Expression {
    operator: Operator,
    lhs: Value,
    rhs: Value,
}

impl Expression {
    pub fn from_string(input: &str) -> Option<Expression> {
        let mut chars = input.trim().chars().peekable();
        let mut lhs: Option<Value> = None;
        let mut rhs: Option<Value> = None;
        let mut op: Option<Operator> = None;
        while let Some(char) = chars.next() {
            if char.is_whitespace() {
                continue;
            }
            if char.is_numeric() {
                if lhs.is_none() {
                    lhs = Some(Value::VALUE(char.to_string().parse().unwrap()));
                } else {
                    rhs = Some(Value::VALUE(char.to_string().parse().unwrap()));
                    if chars.peek().is_some() {
                        lhs = Some(Value::EXPRESSION(Box::new(Expression {
                            operator: op?,
                            lhs: lhs?,
                            rhs: rhs?,
                        })));
                        op = None;
                        rhs = None;
                    }
                }
            } else if char == '+' {
                op = Some(Operator::ADD);
            } else if char == '*' {
                op = Some(Operator::MULTIPLY);
            }
        }
        Some(Expression {
            operator: op?,
            lhs: lhs?,
            rhs: rhs?,
        })
    }

    pub fn calculate(&self) -> isize {
        let lhs = match &self.lhs {
            Value::VALUE(v) => { *v }
            Value::EXPRESSION(expression) => {
                expression.calculate()
            }
        };
        let rhs = match &self.rhs {
            Value::VALUE(v) => { *v }
            Value::EXPRESSION(expression) => {
                expression.calculate()
            }
        };
        match self.operator {
            Operator::ADD => { lhs + rhs }
            Operator::MULTIPLY => { lhs * rhs }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::day18b::*;

    #[test]
    fn test_expression() {
        let expression = Expression {
            operator: Operator::ADD,
            lhs: Value::VALUE(5),
            rhs: Value::EXPRESSION(Box::new(Expression {
                operator: Operator::ADD,
                lhs: Value::VALUE(10),
                rhs: Value::VALUE(7),
            })),
        };
        println!("{:?}", expression);
        assert_eq!(22, expression.calculate());
    }

    #[test]
    fn test_simple_addition() {
        let expr = Expression::from_string("3 + 5");
        assert_eq!(8, expr.unwrap().calculate());
    }

    #[test]
    fn test_three_addition() {
        let expr = Expression::from_string("3+4+5");
        assert_eq!(12, expr.unwrap().calculate());
    }

    #[test]
    fn test_simple_multiplication() {
        let expr = Expression::from_string("3 * 5");
        assert_eq!(15, expr.unwrap().calculate());
    }

    #[test]
    fn test_three_mutliplication() {
        let expr = Expression::from_string("3*4*5");
        assert_eq!(60, expr.unwrap().calculate());
    }

    #[test]
    fn test_l2r_addition_and_multiplication() {
        let expr = Expression::from_string("3*4+5");
        assert_eq!(17, expr.unwrap().calculate());
        let expr = Expression::from_string("3+4*5");
        assert_eq!(35, expr.unwrap().calculate());
    }

    #[test]
    fn test_calculate() {
        /*assert_eq!(51, calculate("1 + (2 * 3) + (4 * (5 + 6))"));
        assert_eq!(26, calculate("2 * 3 + (4 * 5)"));
        assert_eq!(437, calculate("5 + (8 * 3 + 9 + 3 * 4 * 3)"));
        assert_eq!(12240, calculate("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"));
        assert_eq!(13632, calculate("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"));*/
    }
}