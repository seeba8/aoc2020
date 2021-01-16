#[derive(Debug)]
pub enum Operator {
    ADD,
    MULTIPLY,
}

#[allow(dead_code)]
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

#[allow(dead_code)]
fn get_sum_of_lines(input: &str) -> isize {
    let mut sum = 0;
    for line in input.lines() {
        sum += calculate(line);
    }
    sum
}


#[cfg(test)]
mod tests {
    use crate::day18a::*;

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