use std::collections::{HashSet, HashMap};

fn count_distinct_answers(group: &str) -> usize {
    group.chars().filter(|c| c.is_ascii_alphabetic()).collect::<HashSet<_>>().len()
}

pub fn get_sum_of_distinct_answers(input: &str) -> usize {
    input.trim().split("\n\n").map(|group| count_distinct_answers(group)).sum()
}

#[allow(dead_code)]
fn part1_oneliner(input: &str) -> usize {
    input
        .trim()
        .split("\n\n")
        .map(|group| {
            group
                .chars()
                .filter(|c| c.is_alphabetic())
                .collect::<HashSet<_>>()
                .len()
        })
        .sum()
}

fn count_common_answers(group: &str) -> usize {
    let mut num_participants = 0;
    let mut distinct_answers: HashMap<char, usize> = HashMap::new();
    for participant in group.split('\n') {
        num_participants += 1;
        for chr in participant.chars() {
            let val = match distinct_answers.get(&chr) {
                None => { 1 }
                Some(answer) => { answer + 1 }
            };
            distinct_answers.insert(chr, val);
        }
    }
    distinct_answers.iter().filter(|(_, &v)| v == num_participants).count()
}

pub fn get_sum_of_common_answers(input: &str) -> usize {
    input.trim().split("\n\n").map(|group| count_common_answers(group)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let input = r"abc

a
b
c

ab
ac

a
a
a
a

b
";
        let mut iter = input.trim().split("\n\n");
        assert_eq!(3, count_distinct_answers(iter.next().unwrap()));
        assert_eq!(3, count_distinct_answers(iter.next().unwrap()));
        assert_eq!(3, count_distinct_answers(iter.next().unwrap()));
        assert_eq!(1, count_distinct_answers(iter.next().unwrap()));
        assert_eq!(1, count_distinct_answers(iter.next().unwrap()));
        assert_eq!(None, iter.next());

        assert_eq!(11, get_sum_of_distinct_answers(input));
    }

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("resources/day6.txt")
            .expect("Error reading file to string");
        println!("{}", get_sum_of_distinct_answers(input.as_str()));
        println!("{}", part1_oneliner(input.as_str()));
    }

    #[test]
    fn test_part2_example() {
        let input = r"abc

a
b
c

ab
ac

a
a
a
a

b
";
        let mut iter = input.trim().split("\n\n");
        assert_eq!(3, count_common_answers(iter.next().unwrap()));
        assert_eq!(0, count_common_answers(iter.next().unwrap()));
        assert_eq!(1, count_common_answers(iter.next().unwrap()));
        assert_eq!(1, count_common_answers(iter.next().unwrap()));
        assert_eq!(1, count_common_answers(iter.next().unwrap()));
        assert_eq!(None, iter.next());

        assert_eq!(6, get_sum_of_common_answers(input));
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("resources/day6.txt")
            .expect("Error reading file to string");
        println!("{}", get_sum_of_common_answers(input.as_str()));
    }
}