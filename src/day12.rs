pub enum Direction {
    NORTH = 0,
    EAST = 90,
    SOUTH = 180,
    WEST = 270,
}

impl Direction {
    fn turn_right(self, degrees: i64) -> Direction {
        Direction::from_degrees(self as i64 + degrees)
    }
    fn turn_left(self, degrees: i64) -> Direction {
        Direction::from_degrees(self as i64 - degrees)
    }

    fn from_degrees(degrees: i64) -> Direction {
        match ((degrees % 360) + 360) % 360 {
            0 => Direction::NORTH,
            90 => Direction::EAST,
            180 => Direction::SOUTH,
            270 => Direction::WEST,
            _ => panic!("{}", degrees)
        }
    }
}

fn navigate(input: &str) -> (i64, i64) {
    let mut position = (0, 0);
    let mut direction = Direction::EAST;
    for instruction in input.trim().split('\n') {
        let (action, distance) = instruction.split_at(1);
        let distance: i64 = distance.parse().unwrap();
        match action {
            "N" => position.1 += distance,
            "S" => position.1 -= distance,
            "E" => position.0 += distance,
            "W" => position.0 -= distance,
            "L" => direction = direction.turn_left(distance),
            "R" => direction = direction.turn_right(distance),
            "F" => match direction {
                Direction::NORTH => { position.1 += distance }
                Direction::EAST => { position.0 += distance }
                Direction::SOUTH => { position.1 -= distance }
                Direction::WEST => { position.0 -= distance }
            }
            &_ => {}
        }
    }

    position
}


fn navigate_new_instructions(input: &str) -> (i64, i64) {
    let mut position = (0, 0);
    let mut waypoint: (i64, i64) = (10, 1);
    for instruction in input.trim().split('\n') {
        let (action, distance) = instruction.split_at(1);
        let distance: i64 = distance.parse().unwrap();
        match action {
            "N" => waypoint.1 += distance,
            "S" => waypoint.1 -= distance,
            "E" => waypoint.0 += distance,
            "W" => waypoint.0 -= distance,
            "R" => waypoint = rotate_waypoint(waypoint, distance),
            "L" => waypoint = rotate_waypoint(waypoint, 360 - distance),
            "F" => position = (position.0 + distance * waypoint.0, position.1 + distance * waypoint.1),
            _ => {}
        }
    }

    position
}

fn rotate_waypoint(waypoint: (i64, i64), angle: i64) -> (i64, i64) {
    match ((angle % 360) + 360) % 360 {
        90 => (waypoint.1, -waypoint.0),
        180 => (-waypoint.0, -waypoint.1),
        270 => (-waypoint.1, waypoint.0),
        _ => { waypoint }
    }
}

fn get_manhattan_distance(point: (i64, i64)) -> usize {
    (point.0.abs() + point.1.abs()) as usize
}

pub fn get_travel_distance(input: &str, using_waypoint: bool) -> usize {
    match using_waypoint {
        true => get_manhattan_distance(navigate_new_instructions(input)),
        false => get_manhattan_distance(navigate(input))
    }
}

#[cfg(test)]
mod tests {
    use crate::day12::{navigate, get_manhattan_distance, get_travel_distance};

    #[test]
    fn test_navigate() {
        let input = r"F10
N3
F7
R90
F11";
        assert_eq!((17, -8), navigate(input));
    }

    #[test]
    fn test_manhattan_distance() {
        assert_eq!(25, get_manhattan_distance((17, -8)));
    }

    #[test]
    fn test_example_part1() {
        let input = r"F10
N3
F7
R90
F11";
        assert_eq!(25, get_travel_distance(input, false));
    }

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("resources/day12.txt").unwrap();
        println!("{}", get_travel_distance(input.as_str(), false));
    }

    #[test]
    fn test_example_part2() {
        let input = r"F10
N3
F7
R90
F11";
        assert_eq!(286, get_travel_distance(input, true));
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("resources/day12.txt").unwrap();
        println!("{}", get_travel_distance(input.as_str(), true));
    }
}