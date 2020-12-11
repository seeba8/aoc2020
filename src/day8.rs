use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub struct Instruction {
    operation: Operation,
    argument: isize,
}

impl Instruction {
    /// returns a tuple (accumulator, pointer offset)
    fn execute(&self, accumulator: isize) -> (isize, isize) {
        match self.operation {
            Operation::ACC => {
                (accumulator + self.argument, 1)
            }
            Operation::JMP => {
                (accumulator, self.argument)
            }
            Operation::NOP => {
                (accumulator, 1)
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum Operation {
    ACC,
    JMP,
    NOP,
}

pub fn parse_input(input: &str) -> Vec<Instruction> {
    input.trim().split('\n')
        .map(|x| {
            let (op, arg) = x.split_once(' ').unwrap();
            Instruction {
                operation: match op {
                    "acc" => Operation::ACC,
                    "jmp" => Operation::JMP,
                    &_ => Operation::NOP
                },
                argument: arg.parse().unwrap(),
            }
        })
        .collect()
}

pub fn execute(instructions: &[Instruction]) -> isize {
    let mut pointer: isize = 0;
    let mut accumulator: isize = 0;
    let mut visited_instructions: HashSet<usize> = HashSet::new();
    loop {
        let instr = instructions.get(pointer as usize).unwrap();
        let (new_accumulator, pointer_offset) = instr.execute(accumulator);
        pointer += pointer_offset;
        if !visited_instructions.insert(pointer as usize) {
            return accumulator;
        }
        accumulator = new_accumulator;
    }
}

pub fn execute2(mut instructions: Vec<Instruction>) -> isize {
    for i in 0..instructions.len() {
        let mut pointer: isize = 0;
        let mut accumulator: isize = 0;
        let mut visited_instructions: HashSet<usize> = HashSet::new();
        if instructions.get(i).unwrap().operation == Operation::NOP {
            instructions.get_mut(i).unwrap().operation = Operation::JMP;
        } else if instructions.get(i).unwrap().operation == Operation::JMP {
            instructions.get_mut(i).unwrap().operation = Operation::NOP;
        }
        loop {
            let instr = instructions.get(pointer as usize).unwrap();
            let (new_accumulator, pointer_offset) = instr.execute(accumulator);
            pointer += pointer_offset;
            if !visited_instructions.insert(pointer as usize) {
                break;
            }
            if (pointer as usize) >= instructions.len() {
                return new_accumulator;
            }
            accumulator = new_accumulator;
        }
        if instructions.get(i).unwrap().operation == Operation::NOP {
            instructions.get_mut(i).unwrap().operation = Operation::JMP;
        } else if instructions.get(i).unwrap().operation == Operation::JMP {
            instructions.get_mut(i).unwrap().operation = Operation::NOP;
        }
    }
    panic!("problem");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = r"nop +0
acc +1
jmp +4";
        let mut res: Vec<Instruction> = Vec::new();
        res.push(Instruction {
            operation: Operation::NOP,
            argument: 0,
        });
        res.push(Instruction {
            operation: Operation::ACC,
            argument: 1,
        });
        res.push(Instruction {
            operation: Operation::JMP,
            argument: 4,
        });
        let out = parse_input(input);
        assert_eq!(res, out);
    }

    #[test]
    fn test() {
        let mut accumulator: isize = 0;
        let instr = Instruction {
            operation: Operation::ACC,
            argument: 2,
        };
        let (accumulator, next_instr) = instr.execute(accumulator);
        println!("{}", accumulator);
        println!("{}", next_instr);
    }

    #[test]
    fn test_example_a() {
        let input = r"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        let instructions = parse_input(input);
        assert_eq!(5, execute(&instructions));
    }

    #[test]
    fn test_example_b() {
        let input = r"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        let instructions = parse_input(input);
        assert_eq!(8, execute2(instructions));
    }
}