use std::collections::HashSet;
use std::ops::{Range, Add};

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct Coordinate(isize, isize, isize);

impl Add for Coordinate {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            0: self.0 + rhs.0,
            1: self.1 + rhs.1,
            2: self.2 + rhs.2,
        }
    }
}

impl<'a, 'b> Add<&'b Coordinate> for &'a Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: &'b Coordinate) -> Self::Output {
        Self::Output {
            0: self.0 + rhs.0,
            1: self.1 + rhs.1,
            2: self.2 + rhs.2,
        }
    }
}

fn get_coordinates(input: &str) -> Coordinate {

    // https://www.redblobgames.com/grids/hexagons/
    let mut chars = input.trim().chars();
    let mut position = Coordinate(0, 0, 0);
    while let Some(char) = chars.next() {
        match char {
            'n' => {
                let second_char = chars.next().unwrap();
                match second_char {
                    'e' => {
                        position.0 += 1;
                        position.2 -= 1;
                    }
                    'w' => {
                        position.1 += 1;
                        position.2 -= 1;
                    }
                    _ => panic!()
                }
            }
            's' => {
                let second_char = chars.next().unwrap();
                match second_char {
                    'e' => {
                        position.1 -= 1;
                        position.2 += 1;
                    }
                    'w' => {
                        position.0 -= 1;
                        position.2 += 1;
                    }
                    _ => panic!()
                }
            }
            'e' => {
                position.0 += 1;
                position.1 -= 1;
            }
            'w' => {
                position.0 -= 1;
                position.1 += 1;
            }
            _ => panic!()
        }
    }
    position
}

pub fn flip_tiles(input: &str) -> HashSet<Coordinate> {
    let mut black_tiles = HashSet::new();
    for line in input.trim().lines() {
        let coords = get_coordinates(line);
        if !black_tiles.remove(&coords) {
            black_tiles.insert(coords);
        }
    }
    black_tiles
}

fn tick(black_tiles: HashSet<Coordinate>, survive: &Range<usize>, revive: &Range<usize>) -> HashSet<Coordinate> {
    let offsets: Vec<Coordinate> = "ne,e,se,sw,w,nw"
        .split(',')
        .map(|offset| get_coordinates(offset))
        .collect();
    let mut new_grid: HashSet<Coordinate> = HashSet::new();
    let mut neighbours: HashSet<Coordinate> = HashSet::new();
    for black_tile in black_tiles.iter() {
        let mut count_active_neighbours = 0;
        for offset in offsets.iter() {
            if black_tiles.contains(&(black_tile + offset)) {
                count_active_neighbours += 1;
            } else {
                neighbours.insert(black_tile + offset);
            }
        }

        if survive.contains(&count_active_neighbours) {
            new_grid.insert(*black_tile);
        }
    }

    for neighbour in neighbours.iter() {
        let num_active_neighbours = offsets.iter()
            .filter(|&offset| black_tiles.contains(&(offset + neighbour)))
            .count();
        if revive.contains(&num_active_neighbours) {
            new_grid.insert(*neighbour);
        }
    }
    new_grid
}

pub fn tick_n_times(mut black_tiles: HashSet<Coordinate>, num: usize, survive: &Range<usize>, revive: &Range<usize>) -> HashSet<Coordinate> {
    for _ in 0..num {
        black_tiles = tick(black_tiles, survive, revive);
    }
    black_tiles
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_path() {
        assert_eq!(Coordinate(1, -1, 0), get_coordinates("e"));
        assert_eq!(Coordinate(0, -1, 1), get_coordinates("se"));
        assert_eq!(Coordinate(-1, 0, 1), get_coordinates("sw"));
        assert_eq!(Coordinate(-1, 1, 0), get_coordinates("w"));
        assert_eq!(Coordinate(0, 1, -1), get_coordinates("nw"));
        assert_eq!(Coordinate(1, 0, -1), get_coordinates("ne"));
        assert_eq!(Coordinate(0, 0, 0), get_coordinates("nwwswee"));
    }

    #[test]
    fn test_example1() {
        let input = std::fs::read_to_string("resources/day24_example.txt").unwrap();
        let black_tiles = flip_tiles(&input);
        assert_eq!(10, black_tiles.len());
    }

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("resources/day24.txt").unwrap();
        let black_tiles = flip_tiles(&input);
        println!("{}", black_tiles.len());
    }

    #[test]
    fn test_example2() {
        let input = std::fs::read_to_string("resources/day24_example.txt").unwrap();
        let black_tiles = flip_tiles(&input);
        let black_tiles = tick_n_times(black_tiles, 100, &(1..3), &(2..3));
        assert_eq!(2208, black_tiles.len());
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("resources/day24.txt").unwrap();
        let black_tiles = flip_tiles(&input);
        let black_tiles = tick_n_times(black_tiles, 100, &(1..3), &(2..3));
        println!("{}", black_tiles.len());
    }
}