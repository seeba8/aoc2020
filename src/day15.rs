use std::collections::HashMap;

pub struct Sequence {
    starting_numbers: Vec<usize>,
    last_spoken: HashMap<usize, usize>,
    previous_number: usize,
    next_index: usize,
}

impl Sequence {
    pub fn new(starting_numbers: &[usize]) -> Sequence {
        Sequence {
            starting_numbers: Vec::from(starting_numbers),
            last_spoken: HashMap::new(),
            previous_number: 0,
            next_index: 0,
        }
    }
}

impl Iterator for Sequence {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let val: usize;
        if self.next_index < self.starting_numbers.len() {
            val = self.starting_numbers[self.next_index];
        } else {
            val = match self.last_spoken.get(&self.previous_number) {
                None => 0,
                Some(v) => self.next_index - 1 - v
            };
        }
        if self.next_index > 0 {
            self.last_spoken.insert(self.previous_number, self.next_index - 1);
        }
        self.next_index += 1;
        self.previous_number = val;
        Some(self.previous_number)
    }
}

#[cfg(test)]
mod tests {
    use crate::day15::Sequence;

    #[test]
    fn test_sequence_init() {
        let mut seq = Sequence::new(&[0, 3, 6]);
        assert_eq!(0, seq.next_index);
        assert_eq!(0, seq.previous_number);
        assert_eq!(0, seq.next().unwrap());
        assert_eq!(3, seq.next().unwrap());
        assert_eq!(6, seq.next().unwrap());
    }

    #[test]
    fn test_sequence() {
        let mut seq = Sequence::new(&[0, 3, 6]);
        assert_eq!(0, seq.next().unwrap());
        assert_eq!(3, seq.next().unwrap());
        assert_eq!(6, seq.next().unwrap());
        assert_eq!(0, seq.next().unwrap());
        assert_eq!(3, seq.next().unwrap());
        assert_eq!(3, seq.next().unwrap());
        assert_eq!(1, seq.next().unwrap());
        assert_eq!(0, seq.next().unwrap());
        assert_eq!(4, seq.next().unwrap());
        assert_eq!(0, seq.next().unwrap());
    }

    #[test]
    fn test_sequence_skip() {
        let mut seq = Sequence::new(&[0, 3, 6]).skip(8);
        assert_eq!(4, seq.next().unwrap());
    }

    #[test]
    fn test_part1_examples() {
        let skip = 2019;
        assert_eq!(436, Sequence::new(&[0, 3, 6]).skip(skip).next().unwrap());
        assert_eq!(1, Sequence::new(&[1, 3, 2]).skip(skip).next().unwrap());
        assert_eq!(10, Sequence::new(&[2, 1, 3]).skip(skip).next().unwrap());
        assert_eq!(27, Sequence::new(&[1, 2, 3]).skip(skip).next().unwrap());
        assert_eq!(78, Sequence::new(&[2, 3, 1]).skip(skip).next().unwrap());
        assert_eq!(438, Sequence::new(&[3, 2, 1]).skip(skip).next().unwrap());
        assert_eq!(1836, Sequence::new(&[3, 1, 2]).skip(skip).next().unwrap());
    }

    #[test]
    fn test_part1() {
        println!("2020th: {}", Sequence::new(&[20, 9, 11, 0, 1, 2]).skip(2019).next().unwrap());
    }

    #[test] #[ignore] // takes one minute...
    fn test_part2_examples() {
        let skip = 30_000_000 - 1;
        println!("Warning: This will take about 1 minute");
        assert_eq!(175594, Sequence::new(&[0, 3, 6]).skip(skip).next().unwrap());
    }

    #[test] #[ignore] // takes one minute...
    fn test_part2() {
        println!("Warning: This will take about 1 minute");
        println!("30millionth: {}", Sequence::new(&[20, 9, 11, 0, 1, 2]).nth(30_000_000 - 1).unwrap());
    }
}