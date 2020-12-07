use regex::Regex;
use std::collections::HashSet;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(?:(\w+ \w+) bag)").unwrap();
    static ref RE2: Regex = Regex::new(r"(?:(\d+ \w+ \w+) bag)").unwrap();
}

pub fn get_number_of_possible_bags(input: &str, search: &str) -> usize {
    let mut can_contain_gold: HashSet<&str> = HashSet::new();
    let mut changed = true;
    while changed {
        //println!("next");
        changed = false;
        for row in input.trim().split('\n') {
            let mut matches = RE.captures_iter(row);
            let outer = matches.next().unwrap().get(1).unwrap().as_str();
            for bag in matches {
                let inner = bag.get(1).unwrap().as_str();
                if (inner == search || can_contain_gold.contains(inner)) && can_contain_gold.insert(outer) {
                    changed = true;
                }
            }
        }
    }
    //println!("{:#?}", canContainGold);
    can_contain_gold.len()
}

pub fn get_number_of_contained_bags(input: &str, bag: &str) -> usize {
    for row in input.trim().split('\n') {
        if !row.starts_with(bag) {
            continue;
        }
        let matches = RE2.captures_iter(row);
        let mut sum = 0;
        for bag in matches {
            let inner = bag.get(1).unwrap().as_str();

            let (amount, bag) = inner.split_once(' ').unwrap();
            let amount: usize = amount.parse().unwrap();
            if inner == "no other" {
                return 0;
            }
            sum += amount * (1 + get_number_of_contained_bags(input, bag));
        }
        return sum;
    }
    panic!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = r"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 9 faded blue bags, 2 shiny gold bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
";
        assert_eq!(4, get_number_of_possible_bags(input, "shiny gold"));
    }

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("resources/day7.txt")
            .expect("Error reading file to string");
        println!("{}", get_number_of_possible_bags(input.as_str(), "shiny gold"));
    }

    #[test]
    fn test_part2_examples() {
        let input = r"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.
";
        assert_eq!(126, get_number_of_contained_bags(input, "shiny gold"));

        let input = r"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        assert_eq!(32, get_number_of_contained_bags(input, "shiny gold"));
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("resources/day7.txt")
            .expect("Error reading file to string");
        println!("{}", get_number_of_contained_bags(input.as_str(), "shiny gold"));
    }
}
