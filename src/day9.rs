pub fn get_error(input: &str, preamble_length: usize) -> usize {
    let input = parse_input(input);
    for (index, value) in input.iter().enumerate().skip(preamble_length) {
        if !is_sum_of_previous_elems(&input, preamble_length, index, *value) {
            return *value;
        }
    }
    panic!("all numbers are good");
}

fn parse_input(input: &str) -> Vec<usize> {
    let vec: Vec<usize> = input.trim()
        .split('\n')
        .map(|x| x.parse().unwrap())
        .collect();
    vec
}

fn is_sum_of_previous_elems(data: &[usize], preamble_length: usize, index: usize, value: usize) -> bool {
    for i in (index - preamble_length)..(index - 1) {
        for j in i + 1..index {
            if data.get(i).unwrap() + data.get(j).unwrap() == value {
                return true;
            }
        }
    }
    false
}

pub fn get_delta_of_contiguous_set(data: &str, target_sum: usize) -> usize {
    let data = parse_input(data);
    for start in 0..data.len() - 1 {
        for end in start + 1..data.len() {
            let s: usize = data[start..end].iter().sum();
            if s == target_sum {
                println!("start: {}, end: {}", start, end);
                return data[start..end].iter().max().unwrap() + data[start..end].iter().min().unwrap();
            }
        }
    }
    panic!("no suitable delta found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = r"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

        assert_eq!(127, get_error(input, 5));
    }

    #[test]
    fn test_part1() {
        println!("part1: {}", get_error(std::fs::read_to_string("resources/day9.txt").unwrap().as_str(), 25));
    }

    #[test]
    fn test_part2_example() {
        let input = r"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        assert_eq!(62, get_delta_of_contiguous_set(input, 127));
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("resources/day9.txt").unwrap();
        println!("part2: {}",
                 get_delta_of_contiguous_set(input.as_str(),
                                             get_error(input.as_str(), 25)));
    }
}