pub fn get_earliest_bus(input: &str) -> (u64, u64) {
    let (arrival, busses) = input.trim().split_once('\n').unwrap();
    let arrival: u64 = arrival.parse().unwrap();
    let busses: Vec<u64> = busses
        .split(',')
        .filter(|&x| x != "x")
        .map(|x| x.parse().unwrap())
        .collect();

    let mut best_wait = u64::max_value();
    let mut best_bus = 0;
    for bus in busses {
        let bus_iteration = arrival / bus + 1;
        let wait_time = bus_iteration * bus - arrival;
        if wait_time < best_wait {
            best_wait = wait_time;
            best_bus = bus;
        }
    }
    (best_bus, best_wait)
}

pub fn get_timestamp_sequence(input: &str) -> u64 {
    let (_, busses) = input.trim().split_once('\n').unwrap();
    get_timestamp_sequence_busses(busses)
}

fn get_timestamp_sequence_busses(busses: &str) -> u64 {
    let busses: Vec<u64> = busses
        .split(',')
        .map(|x| if x == "x" { 1 } else { x.parse().unwrap() })
        .collect();

    let mut max_bus_id = 0;
    let mut bus_offset: u64 = 0;

    for (index, &bus) in busses.iter().enumerate() {
        if bus > max_bus_id {
            max_bus_id = bus;
            bus_offset = index as u64;
        }
    }
    let increment = max_bus_id;
    let mut t =  max_bus_id - bus_offset;
    'outer: loop {
        t += increment;
        if t % 100_000_000 == 0 {
            println!("{}", t);
        }
        for (offset, &bus) in busses.iter().enumerate() {
            if (t + offset as u64) % bus != 0 { continue 'outer;}
        }
        break;
    }
    t
}

#[cfg(test)]
mod tests {
    use crate::day13::{get_earliest_bus, get_timestamp_sequence, get_timestamp_sequence_busses};

    #[test]
    fn test_get_earliest_bus() {
        let input = r"939
7,13,x,x,59,x,31,19
";
        assert_eq!((59, 5), get_earliest_bus(input));
    }

    #[test]
    fn test_example1() {
        let input = r"939
7,13,x,x,59,x,31,19
";
        let (bus, wait) = get_earliest_bus(input);
        assert_eq!(295, bus * wait);
    }

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("resources/day13.txt").unwrap();
        let (bus, wait) = get_earliest_bus(input.as_str());
        println!("best bus: {}, wait: {}\n => {}", bus, wait, bus * wait);
    }

    #[test]
    fn test_part2_example1() {
        let input = r"939
7,13,x,x,59,x,31,19
";
        assert_eq!(1068781, get_timestamp_sequence(input));
    }

    #[test]
    fn test_part2_examples() {
        assert_eq!(3417, get_timestamp_sequence_busses("17,x,13,19"));
        assert_eq!(754018, get_timestamp_sequence_busses("67,7,59,61"));
        assert_eq!(779210, get_timestamp_sequence_busses("67,x,7,59,61"));
        assert_eq!(1261476, get_timestamp_sequence_busses("67,7,x,59,61"));
        assert_eq!(1202161486, get_timestamp_sequence_busses("1789,37,47,1889"));
    }

    #[test]
    fn test_part2() {
         let input = std::fs::read_to_string("resources/day13.txt").unwrap();
        println!("{}", get_timestamp_sequence(input.as_str()));
    }
}