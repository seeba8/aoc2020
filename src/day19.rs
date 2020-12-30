use std::collections::HashMap;
use pcre2::bytes::Regex;

fn get_rules(input: &str) -> Option<HashMap<u16, &str>> {
    let rules = input.split_once("\n\n")?.0;
    let mut rules_map: HashMap<u16, &str> = HashMap::new();
    for rule in rules.trim().lines() {
        let (id, rule) = rule.trim().split_once(": ")?;
        let id: u16 = id.parse().ok()?;
        rules_map.insert(id, rule);
    }
    Some(rules_map)
}


fn resolve_subrules(rule: &str, rules: &HashMap<u16, &str>, special: bool) -> Option<String> {
    let mut working_copy = format!(" {} ", rule);
    for segment in rule.split_whitespace() {
        if let Ok(rule_id) = segment.parse::<u16>() {
            let mut subrule = resolve_subrules(rules.get(&rule_id)?, rules, special)?;
            if special {
                if rule_id == 8 {
                    subrule = format!(" (?:{})+ ", subrule);
                } else if rule_id == 11 {
                    let sub42 = resolve_subrules(rules.get(&42_u16)?, rules, special)?;
                    let sub31 = resolve_subrules(rules.get(&31_u16)?, rules, special)?;
                    subrule = format!(" (?'eleven' (?:{}) (?P>eleven)? (?:{}) )", sub42, sub31);
                } else if subrule.len() > 1 {
                    subrule = format!(" (?:{}) ", subrule);
                } else {
                    subrule = format!(" {} ", subrule);
                }
            } else if subrule.len() > 1 {
                subrule = format!(" (?:{}) ", subrule);
            } else {
                subrule = format!(" {} ", subrule);
            }


            working_copy = working_copy.replace(&format!(" {} ", segment),
                                                &subrule);
        }
    }
    Some(working_copy.replace(" ", "").replace("\"", ""))
}

fn get_combined_rule(rules: &HashMap<u16, &str>, special: bool) -> Option<String> {
    Some(format!("^{}$", resolve_subrules(rules.get(&0_u16).unwrap(), &rules, special)?))
}

fn get_messages(input: &str) -> Option<Vec<&str>> {
    Some(input.split_once("\n\n")?.1.lines().collect())
}

pub fn get_number_of_matching_messages(input: &str, special: bool) -> Option<usize> {
    let messages = get_messages(input)?;
    let rules = get_rules(input)?;
    let combined_rule = get_combined_rule(&rules, special)?;
    let re: Regex = Regex::new(&combined_rule).unwrap();
    Some(messages.iter().filter(|&msg| re.is_match(msg.as_bytes()).unwrap()).count())
}

#[cfg(test)]
mod tests {
    use crate::day19::{get_rules, resolve_subrules, get_combined_rule, get_number_of_matching_messages};
    use std::collections::HashMap;

    fn get_example1() -> String {
        String::from(r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb
"#)
    }

    fn get_example2() -> String {
        String::from(r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#)
    }

    #[test]
    fn test_get_rules() {
        let input = get_example1();
        let mut expected: HashMap<u16, &str> = HashMap::new();
        expected.insert(0, "4 1 5");
        expected.insert(1, "2 3 | 3 2");
        expected.insert(2, "4 4 | 5 5");
        expected.insert(3, "4 5 | 5 4");
        expected.insert(4, r#""a""#);
        expected.insert(5, r#""b""#);
        assert_eq!(Some(expected), get_rules(&input));
    }

    #[test]
    fn test_resolve_subrules_simple() {
        assert_eq!(Some(String::from("a")), resolve_subrules(r#" "a" "#, &HashMap::new(), false));
    }

    #[test]
    fn test_resolve_subrules() {
        let input = get_example1();
        let rules = get_rules(&input).unwrap();
        assert_eq!("^a(?:(?:aa|bb)(?:ab|ba)|(?:ab|ba)(?:aa|bb))b$", get_combined_rule(&rules, false).unwrap());
    }

    #[test]
    fn test_example1() {
        let input = get_example1();
        assert_eq!(Some(2), get_number_of_matching_messages(&input, false));
    }

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("resources/day19.txt").unwrap();
        println!("{:?}", get_number_of_matching_messages(&input, false));
    }

    #[test]
    fn test_example2() {
        let input = get_example2();
        assert_eq!(Some(3), get_number_of_matching_messages(&input, false));
        assert_eq!(Some(12), get_number_of_matching_messages(&input, true));
    }

    #[test]
    fn test_recursive_capturing_groups() {
        let re = pcre2::bytes::Regex::new(r"^(?'AGRP'a(?P>AGRP)*b)$").unwrap();

        assert_eq!(true, re.is_match("aabb".as_bytes()).unwrap());
        assert_eq!(true, re.is_match("aaabbb".as_bytes()).unwrap());
        assert_eq!(true, re.is_match("ab".as_bytes()).unwrap());
        assert_eq!(false, re.is_match("abb".as_bytes()).unwrap());
        assert_eq!(false, re.is_match("aab".as_bytes()).unwrap());
        assert_eq!(false, re.is_match("aabbb".as_bytes()).unwrap());
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("resources/day19.txt").unwrap();
        let rules = get_rules(&input).unwrap();
        println!("{}", get_combined_rule(&rules, true).unwrap());
        println!("{:?}", get_number_of_matching_messages(&input, true));
    }
}