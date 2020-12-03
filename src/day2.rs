use regex::Regex;
use std::error::Error;

pub fn is_valid(input: &str, re: &Regex) -> Result<bool, Box<dyn Error>> {
    let input = input.trim();
    let segments = re.captures(input).ok_or("Applying regex capture failed")?;
    let min: usize = segments.get(1).ok_or("no first segment")?.as_str().parse()?;
    let max: usize = segments.get(2).ok_or("no second segment")?.as_str().parse()?;
    let pattern: &str = segments.get(3).ok_or("no third segment")?.as_str();
    let password = format!("%{}%", segments.get(4).ok_or("no fourth segment")?.as_str());
    let parts_split = password.split(pattern).count();
    Ok(parts_split > min && parts_split - 1 <= max)
}

pub fn is_valid_new_policy(password: &str, re: &Regex) -> Result<bool, Box<dyn Error>> {
    let password = password.trim();
    let segments = re.captures(password).ok_or("Applying regex capture failed")?;
    let min: usize = segments.get(1).ok_or("no first segment")?.as_str().parse()?;
    let max: usize = segments.get(2).ok_or("no second segment")?.as_str().parse()?;
    let pattern: char = segments.get(3).ok_or("no third segment")?.as_str().chars().next().ok_or("pattern is empty")?;
    let password = segments.get(4).ok_or("no fourth segment")?.as_str();
    let chars: Vec<char> = password.chars().collect();
    let first_is_match = *chars.get(min - 1).ok_or("min value is zero")? == pattern;
    let second_is_match = *chars.get(max - 1).ok_or("max value is higher than password length")? == pattern;
    Ok(first_is_match ^ second_is_match)
}

pub fn get_regex() -> Regex {
    Regex::new(r"(\d+)-(\d+) (\w): (\w+)").expect("Invalid regex")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        let re = get_regex();
        assert_eq!(true, is_valid("1-3 a: abcde", &re).unwrap(), "1-3 a: abcde");
        assert_eq!(false, is_valid("1-3 b: cdefg", &re).unwrap(), "1-3 b: cdefg");
        assert_eq!(true, is_valid("2-9 c: ccccccccc", &re).unwrap(), "2-9 c: ccccccccc");
    }


    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("resources/day2.txt")
            .expect("Error reading file to string");
        let re = get_regex();
        let mut valid_pws = 0;
        for entry in input.trim().split("\n") {
            if is_valid(entry, &re).unwrap() {
                valid_pws += 1;
            }
        }
        println!("{}", valid_pws);
    }

    #[test]
    fn test_part2_examples() {
        assert_eq!(true, is_valid_new_policy("1-3 a: abcde", &get_regex()).unwrap(), "1-3 a: abcde");
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("resources/day2.txt")
            .expect("Error reading file to string");
        let re = get_regex();
        let mut valid_pws = 0;
        for entry in input.trim().split("\n") {
            if is_valid_new_policy(entry, &re).unwrap() {
                valid_pws += 1;
            }
        }
        println!("{}", valid_pws);
    }
}