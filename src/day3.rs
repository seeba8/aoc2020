use std::error::Error;

pub fn slide_down(map: &str, right: usize, down: usize) -> Result<usize, Box<dyn Error>> {
    let map = map.trim();
    let mut position_x = 0;
    let mut num_trees: usize = 0;
    let map_width = map.find('\n').ok_or("No line breaks in map")?;
    for row in map.split('\n').step_by(down) {
        if row.chars().nth(position_x).ok_or_else(|| format!("illegal index: {}", position_x))? == '#' {
            num_trees += 1;
        }
        position_x = (position_x + right) % map_width;
    }
    Ok(num_trees)
}

pub fn multiply_slopes(map: &str) -> Result<usize, Box<dyn Error>> {
    let mut product = 1;
    product *= slide_down(map, 1, 1)?;
    product *= slide_down(map, 3, 1)?;
    product *= slide_down(map, 5, 1)?;
    product *= slide_down(map, 7, 1)?;
    product *= slide_down(map, 1, 2)?;
    Ok(product)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let map = r"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
";
        assert_eq!(7, slide_down(map, 3, 1).unwrap());
    }

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("resources/day3.txt")
            .expect("Error reading file to string");
        println!("{}", slide_down(input.as_str(), 3, 1).expect("Error executing slide_down"));
    }

    #[test]
    fn test_part2_examples() {
        let map = r"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
";
        assert_eq!(2, slide_down(map, 1, 1).unwrap());
        assert_eq!(7, slide_down(map, 3, 1).unwrap());
        assert_eq!(3, slide_down(map, 5, 1).unwrap());
        assert_eq!(4, slide_down(map, 7, 1).unwrap());
        assert_eq!(2, slide_down(map, 1, 2).unwrap());
        assert_eq!(336, multiply_slopes(map).unwrap());
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("resources/day3.txt")
            .expect("Error reading file to string");
        println!("{}", multiply_slopes(input.as_str()).unwrap());
    }
}
