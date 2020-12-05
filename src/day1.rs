pub fn day1a(input: &str) -> Option<usize> {
    let expenses: Vec<usize> = input
        .trim()
        .split('\n')
        .map(|x| {
            x.parse().unwrap()
        })
        .collect();
    for x in &expenses {
        for y in &expenses {
            if x + y == 2020 {
                return Some(x * y);
            }
        }
    }
    None
}

pub fn day1b(input: &str) -> Option<usize> {
    let expenses: Vec<usize> = input
        .trim()
        .split('\n')
        .map(|x| {
            x.parse().unwrap()
        })
        .collect();
    for x in expenses.iter() {
        for y in expenses.iter() {
            for z in expenses.iter() {
                if x + y + z == 2020 {
                    return Some(x * y * z);
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1a_example() {
        let input = "1721\n979\n366\n299\n675\n1456".to_string();
        assert_eq!(514579, day1a(&*input).unwrap());
    }

    #[test]
    fn test_day1b_example() {
        let input = "1721\n979\n366\n299\n675\n1456".to_string();
        assert_eq!(241861950, day1b(&*input).unwrap());
    }

    #[test]
    fn test_day1a() {
        let input = std::fs::read_to_string("resources/day1.txt")
            .expect("Error reading file to string");
        println!("part a: {}", day1a(&*input).unwrap());
    }

    #[test]
    fn test_day1b() {
        let input = std::fs::read_to_string("resources/day1.txt")
            .expect("Error reading file to string");
        println!("part b: {}", day1b(&*input).unwrap());
    }
}
