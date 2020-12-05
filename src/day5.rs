use std::error::Error;

fn get_seat_id(boarding_pass: &str) -> Result<usize, Box<dyn Error>> {
    let (row, col) = boarding_pass.split_at(7);
    let row = usize::from_str_radix(
        row.replace('F', "0")
            .replace('B', "1").as_str(), 2)?;
    let col = usize::from_str_radix(
        col.replace('L', "0")
            .replace('R', "1").as_str(), 2)?;
    Ok(8 * row + col)
}

fn get_highest_id(boarding_passes: &str) -> Result<usize, Box<dyn Error>> {
    Ok(boarding_passes.trim().split('\n')
        .filter_map(|pass| get_seat_id(pass).ok())
        .max().ok_or("Error calculating max")?)
}

fn get_missing_id(boarding_passes: &str) -> Result<usize, &str> {
    let mut passes: Vec<usize> = boarding_passes.trim().split('\n')
        .filter_map(|pass| get_seat_id(pass).ok())
        .collect();
    passes.sort_unstable();
    let mut previous_seat = *passes.get(0).ok_or("test")?;
    for seat in passes.iter().skip(1) {
        if previous_seat + 1 != *seat {
            return Ok(*seat - 1);
        }
        previous_seat = *seat;
    }
    Err("no gap found")
}

#[cfg(test)]
mod tests {
    use crate::day5::{get_seat_id, get_highest_id, get_missing_id};

    #[test]
    fn test_get_seat_id() {
        assert_eq!(357, get_seat_id("FBFBBFFRLR").unwrap());
        assert_eq!(567, get_seat_id("BFFFBBFRRR").unwrap());
        assert_eq!(119, get_seat_id("FFFBBBFRRR").unwrap());
        assert_eq!(820, get_seat_id("BBFFBBFRLL").unwrap());
    }

    #[test]
    fn test_get_highest_id() {
        let passes = r"FBFBBFFRLR
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL
";
        assert_eq!(820, get_highest_id(passes).unwrap());
    }

    #[test]
    fn test_part1() {
        println!("{}", get_highest_id(std::fs::read_to_string("resources/day5.txt").unwrap().as_str()).unwrap())
    }

    #[test]
    fn test_part2() {
        println!("{}", get_missing_id(std::fs::read_to_string("resources/day5.txt").unwrap().as_str()).unwrap())
    }
}