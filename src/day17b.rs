use std::collections::HashSet;
use std::ops::Range;

#[derive(Debug, PartialEq, Eq)]
pub struct Grid {
    active_cells: HashSet<Vec<isize>>,
    survive: Range<usize>,
    revive: Range<usize>,
    dimensions: usize,
}

impl Grid {
    pub fn new(initial_state: &str, num_dimensions: usize, survive: Range<usize>, revive: Range<usize>) -> Grid {
        if num_dimensions < 2 {
            panic!("num_dimensions must be >= 2");
        }
        let mut grid: HashSet<Vec<isize>> = HashSet::new();
        for (y, row) in initial_state.trim().split('\n').enumerate() {
            for (x, chr) in row.chars().enumerate() {
                if chr == '#' {
                    let mut vec: Vec<isize> = vec![0_isize; num_dimensions];
                    vec[0] = x as isize;
                    vec[1] = y as isize;
                    grid.insert(vec);
                }
            }
        }
        Grid {
            active_cells: grid,
            survive,
            revive,
            dimensions: num_dimensions,
        }
    }

    pub fn tick_n_times(&mut self, count: usize) {
        for _ in 0..count {
            self.tick();
        }
    }

    pub fn count_active_cells(&self) -> usize {
        self.active_cells.len()
    }

    fn get_neighbours(&self, coords: &[isize]) -> Vec<Vec<isize>> {
        let mut neighbours: Vec<Vec<isize>> = Vec::new();
        for i in 0_u64..(3_u64.pow(self.dimensions as u32)) {
            let mut offset: Vec<isize> = Vec::with_capacity(self.dimensions);
            offset.push(i as isize % 3); // 3 because left neighbour, same column, right neighbour
            for axis in 1..self.dimensions {
                offset.push((i as isize / (3_isize.pow(axis as u32))) % 3);
            }
            if offset.iter().all(|x| *x == 1_isize) {
                continue;
            }
            neighbours.push(offset
                    .iter().enumerate()
                    .map(|(idx, val)| coords[idx] - 1 + val).collect());
        }
        neighbours
    }

    fn tick(&mut self) {
        let mut new_grid = HashSet::new();
        let mut neighbours: HashSet<Vec<isize>> = HashSet::new();
        for coords in self.active_cells.clone() {
            let mut count_active_neighbours: usize = 0;
            for target in self.get_neighbours(&coords) {
                if self.active_cells.contains(&target) {
                    count_active_neighbours += 1;
                } else {
                    neighbours.insert(target);
                }
            }
            if self.survive.contains(&count_active_neighbours) {
                new_grid.insert(coords);
            }
        }

        for coords in neighbours {
            let mut count_active_neighbours: usize = 0;
            for target in self.get_neighbours(&coords) {
                if self.active_cells.contains(&target[..]) {
                    count_active_neighbours += 1;
                }
            }

            if self.revive.contains(&count_active_neighbours) {
                new_grid.insert(coords);
            }
        }

        self.active_cells = new_grid;
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::day17b::*;

    #[test]
    fn test_grid_init() {
        let mut expected: HashSet<Vec<isize>> = HashSet::new();
        expected.insert(vec![1, 0, 0]);
        expected.insert(vec![2, 1, 0]);
        expected.insert(vec![0, 2, 0]);
        expected.insert(vec![1, 2, 0]);
        expected.insert(vec![2, 2, 0]);
        let grid = Grid::new(r".#.
..#
###", 3, 2..4, 3..4);
        assert_eq!(expected, grid.active_cells);
    }

    #[test]
    fn test_example_part1() {
        let input = r".#.
..#
###";
        let mut grid = Grid::new(input, 3, 2..4, 3..4);
        println!("{:?}", grid);
        grid.tick_n_times(6);
        assert_eq!(112, grid.active_cells.len());
    }

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("resources/day17.txt").unwrap();
        let mut grid = Grid::new(&input, 3, 2..4, 3..4);
        grid.tick_n_times(6);
        println!("active cells: {}", grid.count_active_cells());
    }

    #[test]
    fn test_example_part2() {
        let input = r".#.
..#
###";
        let mut grid = Grid::new(input, 4, 2..4, 3..4);
        grid.tick_n_times(6);
        assert_eq!(848, grid.count_active_cells());
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("resources/day17.txt").unwrap();
        let mut grid = Grid::new(&input, 4, 2..4, 3..4);
        grid.tick_n_times(6);
        println!("active cells: {}", grid.count_active_cells());
    }
}

