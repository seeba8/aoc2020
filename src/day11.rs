#[derive(Debug, PartialEq, Clone, Eq)]
pub enum Object {
    FLOOR,
    EMPTY,
    OCCUPIED,
}

/*fn get_neighbours(index: usize, width: usize, height: usize) -> Vec<usize> {
    let index: isize = index as isize;
    let width: isize = width as isize;
    let height: isize = height as isize;
    let x_start = if index % width == 0 { 1 } else { 0 };
    let x_end = if index % width == width - 1 { 2 } else { 3 };
    let y_start = if index / width == 0 { 1 } else { 0 };
    let y_end = if (index / width) == height - 1 { 2 } else { 3 };
    let mut res: Vec<usize> = Vec::with_capacity(8);
    for y in y_start..y_end {
        for x in x_start..x_end {
            if y == 1 && x == 1 { continue; }
            res.push((index + (x - 1) as isize + (width * (y - 1) as isize)) as usize);
        }
    }
    res
}*/

fn get_neighbours(grid: &[Object], index: usize, width: usize, max_distance: usize) -> Vec<usize> {
    let max_distance = max_distance as isize;
    let height = (grid.len() / width) as isize;
    let width = width as isize;
    let index = index as isize;
    let mut res: Vec<usize> = Vec::with_capacity(8);
    let pos_x = (index % width) as isize;
    let pos_y = (index / width) as isize;
    for y_offset in &[-1, 0, 1] {
        for x_offset in &[-1, 0, 1] {
            if *x_offset == 0 && *y_offset == 0 { continue; }
            let mut distance_factor = 0;
            while distance_factor < max_distance {
                distance_factor += 1;
                let test_x = (pos_x + distance_factor * x_offset) as isize;
                let test_y = (pos_y + distance_factor * y_offset) as isize;

                if test_x < 0 || test_y < 0 || test_x >= width as isize || test_y >= height as isize { break; }
                if grid[(test_y * width + test_x) as usize] != Object::FLOOR {
                    res.push((test_y * width + test_x) as usize);
                    break;
                }
            }
        }
    }
    res
}

fn get_number_of_occupied_neighbours(grid: &[Object], width: usize, seat: usize, max_viewdistance: usize) -> usize {
    let neighbours = get_neighbours(grid, seat, width, max_viewdistance);
    neighbours.iter().filter(|&x| grid[*x] == Object::OCCUPIED).count()
}

fn parse_input(input: &str) -> Vec<Object> {
    let mut res = Vec::new();
    for row in input.trim().split('\n') {
        for c in row.chars() {
            res.push(match c {
                '.' => Object::FLOOR,
                'L' => Object::EMPTY,
                '#' => Object::OCCUPIED,
                _ => { panic!() }
            });
        }
    }
    res
}

fn tick(grid: &mut [Object], width: usize, overpopulated_when: usize, max_viewdistance: usize) -> bool {
    let grid_cloned = grid.to_owned();
    for (k, v) in grid.iter_mut().enumerate() {
        match v {
            Object::FLOOR => {}
            Object::EMPTY => {
                if get_number_of_occupied_neighbours(&grid_cloned, width, k, max_viewdistance) == 0 {
                    *v = Object::OCCUPIED;
                }
            }
            Object::OCCUPIED => {
                if get_number_of_occupied_neighbours(&grid_cloned, width, k, max_viewdistance) >= overpopulated_when {
                    *v = Object::EMPTY;
                }
            }
        }
    }
    grid != &grid_cloned[..]
}

fn get_number_of_occupied_seats(grid: &[Object]) -> usize {
    grid.iter().filter(|&x| *x == Object::OCCUPIED).count()
}

pub fn get_number_of_occupied_seats_after_stabilisation(input: &str, overpopulated_when: usize, max_viewdistance: usize) -> usize {
    let mut grid = parse_input(input);
    let width = input.find('\n').unwrap();
    while tick(&mut grid, width, overpopulated_when, max_viewdistance) {}
    get_number_of_occupied_seats(&grid)
}

#[cfg(test)]
mod tests {
    use crate::day11::*;


    #[test]
    fn test_one_tick() {
        let input = get_example1();
        let mut grid = parse_input(input);
        let expected = parse_input(get_example1_after1());
        tick(&mut grid, input.find('\n').unwrap(), 4, 1);
        assert_eq!(expected, grid);
    }

    #[test]
    fn test_two_ticks() {
        let input = get_example1();
        let mut grid = parse_input(input);
        tick(&mut grid, input.find('\n').unwrap(), 4, 1);
        let expected = parse_input(get_example1_after2());
        tick(&mut grid, input.find('\n').unwrap(), 4, 1);
        assert_eq!(expected, grid);
    }


    #[test]
    fn test_example1_stabilise() {
        let input = get_example1();
        let mut grid = parse_input(input);
        let width = input.find('\n').unwrap();
        let mut count = 0;
        while tick(&mut grid, width, 4, 1) { count += 1; }
        assert_eq!(5, count);
    }

    #[test]
    fn test_example1() {
        assert_eq!(37, get_number_of_occupied_seats_after_stabilisation(get_example1(), 4, 1));
    }

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("resources/day11.txt").unwrap();
        println!("{}", get_number_of_occupied_seats_after_stabilisation(input.as_str(), 4, 1));
    }

    #[test]
    fn test_get_line_of_sight() {
        let input = r".............
.L.L.#.#.#.#.
.............";
        let grid = parse_input(input);
        assert_eq!(vec![16], get_neighbours(&grid, 14, 13, isize::max_value() as usize));
        let input = r".##.##.
#.#.#.#
##...##
...L...
##...##
#.#.#.#
.##.##.";
        let grid = parse_input(input);
        assert_eq!(Vec::new() as Vec<usize>, get_neighbours(&grid, 24, 7, isize::max_value() as usize));
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("resources/day11.txt").unwrap();
        println!("{}", get_number_of_occupied_seats_after_stabilisation(input.as_str(), 5, isize::max_value() as usize));
    }

    fn get_example1() -> &'static str {
        r"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"
    }

    fn get_example1_after1() -> &'static str {
        r"#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##"
    }

    fn get_example1_after2() -> &'static str {
        r"#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##"
    }
}