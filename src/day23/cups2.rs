#[derive(Debug)]
pub struct Cups {
    cups: Vec<u32>,
    current_cup: u32,
}

impl Cups {
    pub fn new(input: &str) -> Self {
        let cups: Vec<u32> = vec![0u32; input.trim().len() + 1];
        let mut cup = Cups {
            cups,
            current_cup: 0,
        };
        cup.init_with_values(input);
        cup
    }

    pub fn new_with_length(input: &str, length: usize) -> Self {
        let cups: Vec<u32> = vec![0u32; length + 1];
        let mut cup = Cups {
            cups,
            current_cup: 0,
        };
        cup.init_with_values(input);
        let last = input.chars().last().unwrap().to_digit(10).unwrap();
        cup.cups[last as usize] = (input.len() + 1) as u32;
        for i in input.len() + 1..=length {
            cup.cups[i] = (i + 1) as u32;
        }
        cup.cups[length] = cup.current_cup;
        cup
    }

    fn init_with_values(&mut self, input: &str) {
        let mut chars = input.trim().chars();
        let mut previous_value = chars.next().and_then(|c| c.to_digit(10)).unwrap();
        let first = previous_value;
        for char in chars {
            let value = char.to_digit(10).unwrap();
            self.cups[previous_value as usize] = value;
            previous_value = value;
        }
        self.cups[previous_value as usize] = first;
        self.current_cup = first;
    }

    fn do_move(&mut self) {
        let extracted_cups = self.extract_n_elements_after_current(3);
        let destination_cup = self.get_destination_cup(&extracted_cups);
        let after_destination = self.cups[destination_cup as usize];
        self.cups[destination_cup as usize] = extracted_cups[0];
        self.cups[extracted_cups[2] as usize] = after_destination;
        self.current_cup = self.cups[self.current_cup as usize];
    }

    pub fn do_moves(&mut self, num: usize) {
        for _ in 0..num {
            self.do_move();
        }
    }

    fn extract_n_elements_after_current(&mut self, num: usize) -> Vec<u32> {
        let mut extracted_cups = Vec::with_capacity(3);
        let mut temp_cup = self.current_cup;
        for _ in 0..num {
            temp_cup = self.cups[temp_cup as usize];
            extracted_cups.push(temp_cup);
        }
        temp_cup = self.cups[temp_cup as usize];
        self.cups[self.current_cup as usize] = temp_cup;
        extracted_cups
    }

    fn get_destination_cup(&self, extracted_cups: &[u32]) -> u32 {
        let len = self.cups.len() as u32 - 1;
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
        let mut res = Vec::with_capacity(8);
        let mut cup = self.cups[1];
        for _ in 0..8 {
            res.push(cup);
            cup = self.cups[cup as usize];
        }
        let res: Vec<String> = res.iter().map(|v| v.to_string()).collect();
        res.join("")
    }

    pub fn get_cups_after_one(&self) -> (u32, u32) {
        let first = self.cups[1];
        let second = self.cups[first as usize];
        (first, second)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_cups() {
        let cups = Cups::new("389125467");
        assert_eq!(8, cups.cups[3]);
        assert_eq!(3, cups.cups[7]);
    }

    #[test]
    fn test_get_extracted_cups() {
        let mut cups = Cups::new("389125467");
        assert_eq!(vec![8, 9, 1], cups.extract_n_elements_after_current(3));
        assert_eq!(2, cups.cups[cups.current_cup as usize]);
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
        println!("{:?}", cups.cups);
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
    fn test_big_cups() {
        let cups = Cups::new_with_length("389125467", 1_000_000);
        let first_20 = &cups.cups[0..20];
        assert_eq!(vec![0, 2, 5, 8, 6, 4, 7, 10, 9, 1, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], first_20);
        assert_eq!(3, cups.cups[1_000_000]);
    }

    #[test]
    fn test_part2_example() {
        let mut cups = Cups::new_with_length("389125467", 1_000_000);
        cups.do_moves(10_000_000);
        let first_20 = &cups.cups[0..20];
        println!("{:?}", first_20);
        assert_eq!((934001, 159792), cups.get_cups_after_one());
    }
}