use regex::Regex;
use std::error::Error;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
}

fn is_valid(input: &str) -> Result<bool, Box<dyn Error>> {
    let input = input.trim();
    let segments = RE.captures(input).ok_or("Applying regex capture failed")?;
    let min: usize = segments.get(1).ok_or("no first segment")?.as_str().parse()?;
    let max: usize = segments.get(2).ok_or("no second segment")?.as_str().parse()?;
    let pattern: &str = segments.get(3).ok_or("no third segment")?.as_str();
    let password = format!("%{}%", segments.get(4).ok_or("no fourth segment")?.as_str());
    let parts_split = password.split(pattern).count();
    Ok(parts_split > min && parts_split - 1 <= max)
}

fn is_valid_new_policy(password: &str) -> Result<bool, Box<dyn Error>> {
    let password = password.trim();
    let segments = RE.captures(password).ok_or("Applying regex capture failed")?;
    let min: usize = segments.get(1).ok_or("no first segment")?.as_str().parse()?;
    let max: usize = segments.get(2).ok_or("no second segment")?.as_str().parse()?;
    let pattern: char = segments.get(3).ok_or("no third segment")?.as_str().chars().next().ok_or("pattern is empty")?;
    let password = segments.get(4).ok_or("no fourth segment")?.as_str();
    let chars: Vec<char> = password.chars().collect();
    let first_is_match = *chars.get(min - 1).ok_or("min value is zero")? == pattern;
    let second_is_match = *chars.get(max - 1).ok_or("max value is higher than password length")? == pattern;
    Ok(first_is_match ^ second_is_match)
}

pub fn count_valid_passwords(input: &str) -> usize {
    input.trim().split('\n')
        .filter_map(|x| is_valid(x).ok())
        .filter(|x| *x)
        .count()
}

pub fn count_valid_passwords_new_policy(input: &str) -> usize {
    input.trim().split('\n')
        .filter_map(|x| is_valid_new_policy(x).ok())
        .filter(|x| *x)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        assert_eq!(true, is_valid("1-3 a: abcde").unwrap(), "1-3 a: abcde");
        assert_eq!(false, is_valid("1-3 b: cdefg").unwrap(), "1-3 b: cdefg");
        assert_eq!(true, is_valid("2-9 c: ccccccccc").unwrap(), "2-9 c: ccccccccc");
    }


    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("resources/day2.txt")
            .expect("Error reading file to string");
        println!("{}", count_valid_passwords(input.as_str()));
    }

    #[test]
    fn test_part2_examples() {
        assert_eq!(true, is_valid_new_policy("1-3 a: abcde").unwrap(), "1-3 a: abcde");
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("resources/day2.txt")
            .expect("Error reading file to string");
        println!("{}", count_valid_passwords_new_policy(input.as_str()));
    }
}
