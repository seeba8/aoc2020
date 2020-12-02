use regex::Regex;

pub fn is_valid(input: &str, re: &Regex) -> bool {
    let input = input.trim();
    let segments = re.captures(input).expect("invalid input");
    let min: usize = segments.get(1).unwrap().as_str().parse().unwrap();
    let max: usize = segments.get(2).unwrap().as_str().parse().unwrap();
    let pattern: &str = segments.get(3).unwrap().as_str();
    let mut password = "%".to_owned();
    password.push_str(segments.get(4).unwrap().as_str());
    password.push('%');
    let parts_split = password.split(pattern).count();
    parts_split > min && parts_split - 1 <= max
}

pub fn is_valid_new_policy(password: &str, re: &Regex) -> bool {
    let input = password.trim();
    let segments = re.captures(input).expect("invalid input");
    let min: usize = segments.get(1).unwrap().as_str().parse().unwrap();
    let max: usize = segments.get(2).unwrap().as_str().parse().unwrap();
    let pattern: char = segments.get(3).unwrap().as_str().chars().next().unwrap();
    let password = segments.get(4).unwrap().as_str();
    let chars: Vec<char> = password.chars().collect();
    (*chars.get(min - 1).unwrap() == pattern) ^ (*chars.get(max - 1).unwrap() == pattern)
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
        assert_eq!(true, is_valid("1-3 a: abcde", &re), "1-3 a: abcde");
        assert_eq!(false, is_valid("1-3 b: cdefg", &re), "1-3 b: cdefg");
        assert_eq!(true, is_valid("2-9 c: ccccccccc", &re), "2-9 c: ccccccccc");
    }


    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("resources/day2.txt")
            .expect("Error reading file to string");
        let re = get_regex();
        let mut valid_pws = 0;
        for entry in input.trim().split("\n") {
            if is_valid(entry, &re) {
                valid_pws += 1;
            }
        }
        println!("{}", valid_pws);
    }

    #[test]
    fn test_part2_examples() {
        assert_eq!(true, is_valid_new_policy("1-3 a: abcde", &get_regex()), "1-3 a: abcde");
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("resources/day2.txt")
            .expect("Error reading file to string");
        let re = get_regex();
        let mut valid_pws = 0;
        for entry in input.trim().split("\n") {
            if is_valid_new_policy(entry, &re) {
                valid_pws += 1;
            }
        }
        println!("{}", valid_pws);
    }
}