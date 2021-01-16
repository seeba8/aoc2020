pub struct Cups {
    cups: Vec<u32>,
    current_index: usize,
}

impl Cups {
    pub fn new(input: &str) -> Cups {
        Cups {
            cups: input.trim().chars().map(|c| c.to_digit(10).unwrap()).collect(),
            current_index: 0,
        }
    }
    #[allow(dead_code)]
    pub fn bigger(input: &str) -> Cups {
        let mut cups: Vec<u32> = (1..=1_000_000).collect();
        for (i, v) in input.chars().enumerate() {
            cups[i] = v.to_digit(10).unwrap();
        }
        Cups{
            cups,
            current_index: 0
        }
    }

    fn do_move(&mut self) {
        //self.cups.iter_mut().skip(self.current_index).cycle()
        //drain(self.current_index + 1..self.current_index + 4));
        // println!("{:?}", self.cups);
        let current_cup = self.cups[self.current_index];
        // println!("current cup: {}", current_cup);
        let mut taken: [u32; 3] = [0, 0, 0];
        let mut overlap = 0;
        for i in 0..3 {
            taken[i] = if self.current_index + 1 >= self.cups.len() {
                overlap += 1;
                self.cups.remove(0)
            } else {
                self.cups.remove(self.current_index + 1)
            }
        }
        // println!("pick up: {:?}", taken);
        // println!("without picked: {:?}", self.cups);
        let destination_cup = self.get_index_of_next_lowest(current_cup);
        // println!("destination index: {} ({})", destination_cup, self.cups[destination_cup]);

        for i in (0..3).rev() {
            self.cups.insert(destination_cup + 1, taken[i]);
        }
        if destination_cup < self.current_index {
            self.cups.rotate_left(3 - overlap);
        }
        // println!("{:?}", self.cups);
        self.current_index = (self.current_index + 1) % self.cups.len();
        // println!();
    }

    pub fn do_moves(&mut self, count: usize) {
        for _ in 0..count {
            self.do_move();
        }
    }

    fn get_index_of_next_lowest(&self, current_cup: u32) -> usize {
        let mut current_cup = current_cup;
        let len = self.cups.len() as u32+ 3;
        loop {
            current_cup = (current_cup - 1) % len;
            if current_cup == 0 {
                current_cup = len;
            }
            // println!("{}", current_cup);
            for (key, val) in self.cups.iter().enumerate() {
                if val == &current_cup {
                    return key;
                }
            }
        }
    }

    pub fn get_order(&self) -> String {
        let index_of_1: usize = self.cups.iter().enumerate().map(|(k, v)| if v == &1 { k } else { 0 }).sum();
        self.cups.iter()
            .cycle()
            .skip(index_of_1 + 1)
            .take(8)
            .map(|v| char::from_digit(*v as u32, 10).unwrap())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;
    use crate::day23a::Cups;

    #[test]
    fn test_do_move() {
        let mut cups = Cups {
            cups: vec![3, 8, 9, 1, 2, 5, 4, 6, 7],
            current_index: 0,
        };
        cups.do_move();
        assert_eq!(vec![3, 2, 8, 9, 1, 5, 4, 6, 7], cups.cups);
    }

    #[test]
    fn test_10_moves() {
        let mut cups = Cups {
            cups: vec![3, 8, 9, 1, 2, 5, 4, 6, 7],
            current_index: 0,
        };
        cups.do_moves(10);
        assert_eq!(vec![5, 8, 3, 7, 4, 1, 9, 2, 6], cups.cups);
    }

    #[test]
    fn test_get_order() {
        let mut cups = Cups {
            cups: vec![3, 8, 9, 1, 2, 5, 4, 6, 7],
            current_index: 0,
        };
        cups.do_moves(10);
        assert_eq!(String::from("92658374"), cups.get_order());
    }

    #[test]
    fn test_example_100() {
        // let mut cups = Cups {
        //     cups: vec![3, 8, 9, 1, 2, 5, 4, 6, 7],
        //     current_index: 0,
        // };
        let mut cups = Cups::new("389125467");
        cups.do_moves(100);
        assert_eq!("67384529", cups.get_order());
    }

    #[test]
    fn test_part1() {
        let mut cups = Cups::new("467528193");
        cups.do_moves(100);
        println!("part1 : {}", cups.get_order());
    }

    #[test]
    fn test_get_bigger() {
        let cups = Cups::bigger("467528193");
        println!("{:?}", cups.cups.iter().take(100).collect::<Vec<&u32>>());
    }

    #[ignore] #[test]
    fn test_example_part2() {
        let mut cups = Cups::bigger("467528193");
        let now = Instant::now();
        cups.do_moves(10_000_000);
        println!("{}", now.elapsed().as_millis());
    }
}