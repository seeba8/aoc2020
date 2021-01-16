use std::collections::HashSet;
use std::ops::Range;

#[derive(Debug, PartialEq, Eq)]
pub struct Grid {
    pub active_cells: HashSet<(isize, isize, isize)>,
    pub survive: Range<usize>,
    pub revive: Range<usize>,
}

#[allow(dead_code)]
impl Grid {
    pub fn new(initial_state: &str, survive: Range<usize>, revive: Range<usize>) -> Grid {
        let mut grid = HashSet::new();
        for (y, row) in initial_state.trim().split('\n').enumerate() {
            for (x, chr) in row.chars().enumerate() {
                if chr == '#' {
                    grid.insert((x as isize, y as isize, 0_isize));
                }
            }
        }
        Grid {
            active_cells: grid,
            survive,
            revive,
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

    fn tick(&mut self) {
        let mut new_grid = HashSet::new();
        let mut neighbours: HashSet<(isize, isize, isize)> = HashSet::new();
        for &(x, y, z) in self.active_cells.iter() {
            let mut count_active_neighbours: usize = 0;
            for x_offset in 0..3 {
                for y_offset in 0..3 {
                    for z_offset in 0..3 {
                        if x_offset == 1 && y_offset == 1 && z_offset == 1 {
                            continue;
                        }
                        if self.active_cells.contains(&(x + x_offset - 1,
                                                        y + y_offset - 1,
                                                        z + z_offset - 1)) {
                            count_active_neighbours += 1;
                        } else {
                            neighbours.insert((x + x_offset - 1,
                                               y + y_offset - 1,
                                               z + z_offset - 1));
                        }
                    }
                }
            }
            if self.survive.contains(&count_active_neighbours) {
                new_grid.insert((x, y, z));
            }
        }
        for &(x, y, z) in neighbours.iter() {
            let mut count_active_neighbours: usize = 0;
            for x_offset in 0..3 {
                for y_offset in 0..3 {
                    for z_offset in 0..3 {
                        if x_offset == 1 && y_offset == 1 && z_offset == 1 {
                            continue;
                        }
                        if self.active_cells.contains(&(x + x_offset - 1,
                                                        y + y_offset - 1,
                                                        z + z_offset - 1)) {
                            count_active_neighbours += 1;
                        }
                    }
                }
            }
            if self.revive.contains(&count_active_neighbours) {
                new_grid.insert((x, y, z));
            }
        }

        self.active_cells = new_grid;
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::day17a::*;

    #[test]
    fn test_grid_init() {
        let mut expected: HashSet<(isize, isize, isize)> = HashSet::new();
        expected.insert((1, 0, 0));
        expected.insert((2, 1, 0));
        expected.insert((0, 2, 0));
        expected.insert((1, 2, 0));
        expected.insert((2, 2, 0));
        let grid = Grid::new(r".#.
..#
###", 2..4, 3..4);
        assert_eq!(expected, grid.active_cells);
    }

    #[test]
    fn test_example_part1() {
        let input = r".#.
..#
###";
        let mut grid = Grid::new(input, 2..4, 3..4);
        grid.tick_n_times(6);
        assert_eq!(112, grid.active_cells.len());
    }

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("resources/day17.txt").unwrap();
        let mut grid = Grid::new(&input, 2..4, 3..4);
        grid.tick_n_times(6);
        println!("active cells: {}", grid.count_active_cells());
    }
}

