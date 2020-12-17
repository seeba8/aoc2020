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

pub fn get_timestamp_sequence(input: &str) -> i64 {
    let (_, busses) = input.trim().split_once('\n').unwrap();
    chinese_remainder(busses)
}
/*
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
    let mut t = max_bus_id - bus_offset;
    let busses: Vec<(u16, u16)> = busses
        .iter()
        .enumerate()
        .filter(|(_, &v)| v != 1)
        .map(|(k, &v)| (k as u16, v as u16))
        .collect();
    'outer: loop {
        t += increment;
        if t % 100_000_000 == 0 {
            println!("{}", t);
        }
        for (offset, bus) in busses.iter() {
            if (t + *offset as u64) % *bus as u64 != 0 { continue 'outer; }
        }
        break;
    }
    t
}
*/
#[allow(clippy::many_single_char_names)]
fn eea(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 { return (a, 1, 0); }
    let (d, s_, s) = eea(b, a % b);
    let t = s_ - (a / b) * s;
    (d, s, t)
}

pub fn chinese_remainder(busses: &str) -> i64 {
    let busses: Vec<(i64, i64)> = busses
        .split(',')
        .map(|x| if x == "x" { 1 } else { x.parse().unwrap() })
        .enumerate()
        .filter(|(_, bus)| *bus != 1)
        .map(|(idx, bus)| (idx as i64, bus as i64))
        .map(|(idx, bus)| (bus , ((bus - idx) % bus)))
        .collect();

    let m_all: i64 = busses
        .iter()
        .map(|(bus, _)| *bus)
        .product();
    let m: Vec<i64> = busses
        .iter()
        .map(|(bus, _)| m_all / *bus)
        .collect();
    let eea: Vec<(i64, i64, i64)> = m
        .iter()
        .enumerate()
        .map(|(k, m_)| eea(*m_, busses[k].0))
        .collect();
    let e: Vec<i64> = m
        .iter()
        .enumerate()
        .map(|(k, m_)| *m_ * eea[k].1)
        .collect();
    let remainder = busses
        .iter()
        .enumerate()
        .map(|(idx, (_, offset))| e[idx] * offset)
        .sum::<i64>() % m_all;
    (remainder + m_all) % m_all

}

#[cfg(test)]
mod tests {
    use crate::day13::*;

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
    fn test_part2() {
        let input = std::fs::read_to_string("resources/day13.txt").unwrap();
        println!("{}", get_timestamp_sequence(input.as_str()));
    }

    #[test]
    fn test_chinese_remainder() {
        assert_eq!(3417, chinese_remainder("17,x,13,19"));
        assert_eq!(754018, chinese_remainder("67,7,59,61"));
        assert_eq!(779210, chinese_remainder("67,x,7,59,61"));
        assert_eq!(1261476, chinese_remainder("67,7,x,59,61"));
        assert_eq!(1202161486, chinese_remainder("1789,37,47,1889"));
    }
}