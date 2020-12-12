/// returns the number of 1-jolt differences multiplied by the number of 3-jolt differences
pub fn get_jolt_difference(input: &str) -> usize {
    let vec: Vec<u8> = parse_input(input);
    let mut one_difference = 0;
    let mut three_difference = 0; // already one because of the hop to the device
    let mut previous_joltage: u8 = 0;
    for i in 1..vec.len() {
        let current_joltage = vec.get(i).unwrap();
        match current_joltage - previous_joltage {
            1 => one_difference += 1,
            3 => three_difference += 1,
            _ => {}
        }
        previous_joltage = *current_joltage;
    }

    one_difference * three_difference
}

pub fn get_number_of_possible_arrangements(input: &str) -> u64 {
    let adapters: Vec<u8> = parse_input(input);
    let clusters = get_clusters(&adapters);
    clusters.iter().map(|(start, stop)| {
        if *start == 0 {
            return get_number_of_possible_arrangements_recursive(&adapters[*start..stop+1], 0);
        }
        get_number_of_possible_arrangements_recursive(&adapters[start-1..stop+1], 0)
    })
        .product()
}

fn get_number_of_possible_arrangements_recursive(adapters: &[u8], current_index: usize) -> u64 {//, possibilities: &mut Box<u64>) {
    let current_value = adapters.get(current_index).unwrap();
    let mut num_arrangements = 0;
    // println!("{}", current_value);
    if current_index + 1 == adapters.len() {
        return 1;
    }
    for i in (current_index + 1)..(current_index + 4) {
        if i >= adapters.len() {
            break;
        }
        let possible_value = adapters.get(i).unwrap();
        if possible_value - current_value > 3 {
            return num_arrangements;
        }
        num_arrangements += get_number_of_possible_arrangements_recursive(adapters, i);
    }
    num_arrangements
}

fn parse_input(input: &str) -> Vec<u8> {
    let mut vec: Vec<u8> = input.trim()
        .split('\n')
        .map(|x| x.parse().unwrap())
        .collect();
    vec.sort_unstable();
    vec.insert(0, 0);
    vec.push(vec.last().unwrap() + 3);
    vec
}

fn get_clusters(adapters: &[u8]) -> Vec<(usize, usize)> {
    let mut clusters: Vec<(usize, usize)> = Vec::new();
    let mut start_cluster: Option<usize> = None;
    for i in 0..adapters.len() {
        let current_val = adapters.get(i).unwrap();
        match start_cluster {
            None => {
                // check if we can reach two fields over. If yes, it's a new cluster
                if i+2 < adapters.len() && adapters.get(i+2).unwrap() - current_val <= 3 {
                    start_cluster = Some(i);
                }
            }
            Some(start_val) => {
                // safe because start_cluster can't be Some in the first round
                let prev = adapters.get(i-1).unwrap();
                if i+1 < adapters.len() && adapters.get(i+1).unwrap() - prev > 3 {
                    clusters.push((start_val, i));
                    start_cluster = None;
                }
            }
        }
    }
    clusters
}

#[cfg(test)]
mod tests {
    use crate::day10::{parse_input, get_jolt_difference, get_number_of_possible_arrangements, get_clusters};

    //   use super::*;
    #[test]
    fn test_parse_input() {
        // vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4]
        assert_eq!(vec![0, 1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19, 22], parse_input(get_example1()));
    }

    #[test]
    fn test_jolt_difference() {
        assert_eq!(35, get_jolt_difference(get_example1()));
        assert_eq!(220, get_jolt_difference(get_example2()));
    }

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("resources/day10.txt").unwrap();
        println!("Jolt difference: {}", get_jolt_difference(input.as_str()));
    }

    #[test]
    fn test_get_number_of_possible_arrangements() {
        assert_eq!(8, get_number_of_possible_arrangements(get_example1()));
        assert_eq!(19208, get_number_of_possible_arrangements(get_example2()));
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("resources/day10.txt").unwrap();
        println!("Possible arrangements: {}", get_number_of_possible_arrangements(input.as_str()));
    }

    #[test]
    fn test_get_clusters() {
        let adapters = parse_input(get_example1());
        assert_eq!(vec![(2,5), (6,8)], get_clusters(&adapters));
    }

    fn get_example1() -> &'static str {
        r"16
10
15
5
1
11
7
19
6
12
4
"
    }

    fn get_example2() -> &'static str {
        r"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"
    }
}