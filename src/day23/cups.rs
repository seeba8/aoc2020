use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Cup {
    value: u32,
    next: u32,
}

#[derive(Debug)]
pub struct Cups {
    cups: HashMap<u32, Cup>,
    current_cup: u32,
}

impl Cups {
    pub fn new(input: &str) -> Self {
        let mut cups = HashMap::with_capacity(input.len());
        let mut previous_value: Option<u32> = None;
        let mut first: Option<u32> = None;
        for char in input.trim().chars() {
            let value = char.to_digit(10).unwrap();
            if first.is_none() {
                first = Some(value);
            }
            cups.insert(value, Cup { value, next: 0 });
            match previous_value {
                None => {}
                Some(previous_value) => {
                    match cups.get_mut(&previous_value) {
                        None => { panic!() }
                        Some(cup) => { cup.next = value; }
                    }
                }
            }
            previous_value = Some(value);
        }
        if let Some(last) = cups.get_mut(&previous_value.unwrap()) {
            last.next = first.unwrap();
        };
        Cups {
            cups,
            current_cup: first.unwrap(),
        }
    }

    pub fn new_with_length(input: &str, length: usize) -> Self {
        let mut cups = Cups::new(input);
        cups.cups.extend_reserve(length - input.len());
        let mut previous = input.chars().last().unwrap().to_digit(10).unwrap();
        for i in input.len() as u32 + 1..=length as u32 {
            cups.cups.insert(i, Cup { value: i, next: 0 });
            if let Some(prev) = cups.cups.get_mut(&previous) {
                prev.next = i;
            }
            previous = i;
        }
        if let Some(last) = cups.cups.get_mut(&(length as u32)) {
            last.next = cups.current_cup;
        }
        println!("Init done");
        cups
    }


    fn do_move(&mut self) {
        let extracted_cups = self.extract_n_elements_after_current(3);
        let destination_cup = self.get_destination_cup(&extracted_cups);
        let mut after_destination: Option<u32> = None;
        if let Some(destination_cup) = self.cups.get_mut(&destination_cup) {
            after_destination = Some(destination_cup.next);
            destination_cup.next = *extracted_cups.get(0).unwrap();
        }
        if let Some(last_extracted) = self.cups.get_mut(extracted_cups.last().unwrap()) {
            last_extracted.next = after_destination.unwrap();
        }
        self.current_cup = self.cups.get(&self.current_cup).unwrap().next;
    }

    pub fn do_moves(&mut self, num: usize) {
        for i in 0..num {
            if i % 100_000 == 0 {
                println!("{}", i);
            }
            self.do_move();
        }
    }

    fn extract_n_elements_after_current(&mut self, num: usize) -> Vec<u32> {
        let mut extracted_cups = Vec::with_capacity(3);
        let mut temp_cup = self.current_cup;
        for _ in 0..num {
            temp_cup = self.cups.get(&temp_cup).unwrap().next;
            extracted_cups.push(temp_cup);
        }
        temp_cup = self.cups.get(&temp_cup).unwrap().next;
        if let Some(current_cup) = self.cups.get_mut(&self.current_cup) {
            current_cup.next = temp_cup;
        }
        extracted_cups
    }

    fn get_destination_cup(&self, extracted_cups: &[u32]) -> u32 {
        let len = self.cups.len() as u32;
        let mut destination_cup = self.current_cup - 1;
        if destination_cup == 0 {
            destination_cup = len;
        }
        while extracted_cups.contains(&destination_cup) {
            destination_cup -= 1;
            if destination_cup == 0 {
                destination_cup = len;
            }
        }
        destination_cup
    }

    #[allow(dead_code)]
    pub fn get_order(&self) -> String {
        let mut found_one = false;
        let mut cup = self.cups.get(&self.current_cup).unwrap();
        while !found_one {
            if cup.value == 1 {
                found_one = true;
            }
            cup = self.cups.get(&cup.next).unwrap();
        }
        let mut res = Vec::with_capacity(8);
        for _ in 0..8 {
            res.push(cup.value);
            cup = self.cups.get(&cup.next).unwrap();
        }
        let res: Vec<String> = res.iter().map(|v| v.to_string()).collect();
        res.join("")
    }

    pub fn get_cups_after_one(&self) -> (u32, u32) {
        let first = self.cups.get(&1).unwrap().next;
        let second = self.cups.get(&first).unwrap().next;
        (self.cups.get(&first).unwrap().value, self.cups.get(&second).unwrap().value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_cups() {
        let cups = Cups::new("389125467");
        assert_eq!(Some(&Cup { value: 3, next: 8 }), cups.cups.get(&3));
        assert_eq!(Some(&Cup { value: 7, next: 3 }), cups.cups.get(&7));
    }


    #[test]
    fn test_get_extracted_cups() {
        let mut cups = Cups::new("389125467");
        assert_eq!(vec![8, 9, 1], cups.extract_n_elements_after_current(3));
        assert_eq!(2, cups.cups.get(&cups.current_cup).unwrap().next);
    }

    #[test]
    fn test_do_move() {
        let mut cups = Cups::new("389125467");
        cups.do_move();
        assert_eq!(String::from("54673289"), cups.get_order());
    }

    #[test]
    fn test_get_destination_cup() {
        let mut cups = Cups::new("389125467");
        assert_eq!(9, cups.get_destination_cup(&vec![2, 1]));
        let extracted = cups.extract_n_elements_after_current(3);
        assert_eq!(2, cups.get_destination_cup(&extracted));
    }

    #[test]
    fn test_part1_examples() {
        let mut cups = Cups::new("389125467");
        cups.do_moves(10);
        assert_eq!("92658374", &cups.get_order());
        cups.do_moves(90);
        assert_eq!("67384529", &cups.get_order());
    }

    #[test]
    fn test_part1() {
        let mut cups = Cups::new("467528193");
        cups.do_moves(100);
        assert_eq!("43769582", cups.get_order());
    }

    #[test]
    #[ignore]
    fn test_part2_example() {
        let mut cups = Cups::new_with_length("389125467", 1_000_000);
        cups.do_moves(10_000_000);
        assert_eq!((934001, 159792), cups.get_cups_after_one());
    }
}