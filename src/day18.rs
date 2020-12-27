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
    pub fn new(input: &str) -> Expression {
        let mut lhs: Option<Value> = None;
        let mut rhs: Option<Value> = None;
        let mut op: Option<Operator> = None;
        let mut expression: Option<Expression> = None;
        for chr in input.chars() {
            if chr.is_numeric() {
                if lhs.is_none() {
                    lhs = Some(Value::VALUE(chr.to_string().parse().unwrap()));
                } else {
                    rhs = Some(Value::VALUE(chr.to_string().parse().unwrap()));
                    expression = Some(Expression {
                        operator: op.unwrap(),
                        lhs: lhs.unwrap(),
                        rhs: rhs.unwrap(),
                    });
                    lhs = None;
                    rhs = None;
                    op = None;
                }
            } else if chr == '+' {
                op = Some(Operator::ADD);
            } else if chr == '*' {
                op = Some(Operator::MULTIPLY);
            }
        }
        expression.unwrap()
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

pub fn calculate(input: &str) -> isize {
    let mut input = input.to_owned();
    loop {
        let last_open = input.rfind('(');
        match last_open {
            None => { break; }
            Some(last_open) => {
                let first_close = last_open + input[last_open..].find(')').unwrap();
                input = input.replace(&input[last_open..first_close + 1], &calculate(&input[last_open + 1..first_close]).to_string());
            }
        }
    }
    let mut op: Option<Operator> = None;
    let mut result = 0;
    let mut iter = input.chars().peekable();
    while let Some(chr) = iter.next() {
        if chr == '+' {
            op = Some(Operator::ADD);
        } else if chr == '*' {
            op = Some(Operator::MULTIPLY);
        } else if chr.is_numeric() {
            let mut num = chr.to_string();
            loop {
                if iter.peek().is_some() && iter.peek().unwrap().is_numeric() {
                    num += &iter.next().unwrap().to_string();

                } else {
                    break;
                }
            }
            let num: isize = num.parse().unwrap();
            match &op {
                None => {
                    result = num;
                }
                Some(actual_op) => {
                    match actual_op {
                        Operator::ADD => {
                            result += num;
                        }
                        Operator::MULTIPLY => {
                            result *= num;
                        }
                    }
                    op = None;
                }
            }
        }
    }
    result
}

fn get_sum_of_lines(input: &str) -> isize {
    let mut sum = 0;
    for line in input.lines() {
        sum += calculate(line);
    }
    sum
}



#[cfg(test)]
mod tests {
    use crate::day18::*;

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
    fn test_calculate() {
        assert_eq!(51, calculate("1 + (2 * 3) + (4 * (5 + 6))"));
        assert_eq!(26, calculate("2 * 3 + (4 * 5)"));
        assert_eq!(437, calculate("5 + (8 * 3 + 9 + 3 * 4 * 3)"));
        assert_eq!(12240, calculate("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"));
        assert_eq!(13632, calculate("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"));
    }

    #[test]
    fn test_part1() {
        println!("{}", get_sum_of_lines(&std::fs::read_to_string("resources/day18.txt").unwrap()));
    }
}