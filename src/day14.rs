use std::collections::{HashMap, HashSet};
use regex::Regex;
use std::error::Error;

#[derive(Debug, PartialEq)]
struct Mask {
    and_mask: u64,
    or_mask: u64,
    raw: String,
}

impl Mask {
    fn new(mask: &str) -> Mask {
        Mask {
            and_mask: u64::from_str_radix(mask.replace("X", "1").as_str(), 2).unwrap(),
            or_mask: u64::from_str_radix(mask.replace("X", "0").as_str(), 2).unwrap(),
            raw: mask.to_owned(),
        }
    }

    fn apply(&self, value: u64) -> u64 {
        (value & self.and_mask) | self.or_mask
    }

    fn apply_on_address(&self, address: u64) -> HashSet<u64> {
        let mut addresses: HashSet<u64> = HashSet::new();
        addresses.insert(address | self.or_mask);
        for (idx, char) in self.raw.chars().enumerate() {
            if char == 'X' {
                for a in addresses.clone() {
                    addresses.insert(a | self.or_mask | 2_u64.pow((35 - idx) as u32));
                    addresses.insert((a | self.or_mask) & !(2_u64.pow((35 - idx) as u32)));
                }
            }
        }
        addresses
    }
}

pub struct Decoder {
    mask: Option<Mask>,
    memory: HashMap<u64, u64>,
    version: DecoderVersion,
}

impl Decoder {
    pub fn new(version: DecoderVersion) -> Decoder {
        Decoder {
            mask: None,
            memory: HashMap::new(),
            version,
        }
    }

    pub fn run_programme(&mut self, input: &str) -> Result<(), Box<dyn Error>> {
        match self.version {
            DecoderVersion::V1 => { self._run_programme_v1(input) }
            DecoderVersion::V2 => { self._run_programme_v2(input) }
        }
    }

    pub fn get_sum_of_memory(&self) -> u64 {
        self.memory.iter().map(|(_, v)| *v).sum()
    }

    fn _run_programme_v1(&mut self, input: &str) -> Result<(), Box<dyn Error>> {
        for line in input.trim().split('\n') {
            if line.starts_with("mask = ") {
                self.mask = Some(Mask::new(line.strip_prefix("mask = ").ok_or("Error stripping prefix")?));
            } else {
                let res = &RE.captures(line).ok_or("Error applying regex")?;
                let index: u64 = res.get(1).ok_or("Error getting memory index")?.as_str().parse()?;
                let value: u64 = res.get(2).ok_or("Error getting memory index")?.as_str().parse()?;
                match &self.mask {
                    None => { panic!("Mask is not defined"); }
                    Some(m) => { self.memory.insert(index, m.apply(value)); }
                }
            }
        }
        Ok(())
    }

    fn _run_programme_v2(&mut self, input: &str) -> Result<(), Box<dyn Error>> {
        for line in input.trim().split('\n') {
            if line.starts_with("mask = ") {
                self.mask = Some(Mask::new(line.strip_prefix("mask = ").ok_or("Error stripping prefix")?));
            } else {
                let res = &RE.captures(line).ok_or("Error applying regex")?;
                let index: u64 = res.get(1).ok_or("Error getting memory index")?.as_str().parse()?;
                let value: u64 = res.get(2).ok_or("Error getting memory index")?.as_str().parse()?;
                match &self.mask {
                    None => { panic!("Mask is not defined"); }
                    Some(m) => {
                        for address in m.apply_on_address(index) {
                            self.memory.insert(address, value);
                        }
                    }
                }
            }
        }
        Ok(())
    }
}


pub enum DecoderVersion {
    V1,
    V2,
}


lazy_static! {
    static ref RE: Regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
}


#[cfg(test)]
mod tests {
    use crate::day14::*;

    #[test]
    fn test_binary_number_to_string() {
        assert_eq!(11, u64::from_str_radix("000000000000000000000000000000001011", 2).unwrap());
        assert_eq!("1011", format!("{:b}", 11));
    }

    #[test]
    fn test_get_mask() {
        assert_eq!(Mask { and_mask: 253, or_mask: 64, raw: String::from("X1XXXX0X") }, Mask::new("X1XXXX0X"));
    }

    #[test]
    fn test_apply_mask() {
        assert_eq!(73, Mask::new("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X").apply(11));
        assert_eq!(101, Mask::new("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X").apply(101));
        assert_eq!(64, Mask::new("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X").apply(0));
    }

    #[test]
    fn test_run_programme() {
        let input = r"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
        let mut expected: HashMap<u64, u64> = HashMap::new();
        expected.insert(7, 101);
        expected.insert(8, 64);
        let mut decoder = Decoder::new(DecoderVersion::V1);
        decoder.run_programme(input).unwrap();
        assert_eq!(expected, decoder.memory);
        assert_eq!(165 as u64, decoder.get_sum_of_memory());
    }

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("resources/day14.txt").unwrap();
        let mut decoder = Decoder::new(DecoderVersion::V1);
        decoder.run_programme(input.as_str()).unwrap();
        println!("{}", &decoder.get_sum_of_memory());
    }

    #[test]
    fn test_apply_bitmask_on_address() {
        let mask = Mask::new("000000000000000000000000000000X1001X");
        let mut expected: HashSet<u64> = HashSet::new();
        expected.extend(&[26, 27, 58, 59]);
        assert_eq!(expected, mask.apply_on_address(42));
    }

    #[test]
    fn test_part2_example() {
        let input = r"mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";
        let mut decoder = Decoder::new(DecoderVersion::V2);
        decoder.run_programme(input).unwrap();
        assert_eq!(208, decoder.get_sum_of_memory());
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("resources/day14.txt").unwrap();
        let mut decoder = Decoder::new(DecoderVersion::V2);
        decoder.run_programme(input.as_str()).unwrap();
        println!("{}", decoder.get_sum_of_memory());
    }
}