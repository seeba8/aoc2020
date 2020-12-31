use std::convert::TryFrom;

pub enum Border {
    TOP,
    RIGHT,
    BOTTOM,
    LEFT,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Tile {
    pub id: u16,
    data: [bool; 100],
}

impl Tile {
    pub fn new(input: &str) -> Option<Tile> {
        let (title, data) = input.trim().split_once(":\n")?;
        let tile_id: u16 = title.trim().split_once(' ')?.1.parse().ok()?;
        let mut bitmap = [false; 100];
        let mut idx = 0;
        for c in data.chars() {
            match c {
                '#' => {
                    bitmap[idx] = true;
                    idx += 1;
                }
                '.' => {
                    bitmap[idx] = false;
                    idx += 1
                }
                _ => {}
            }
        }
        Some(Tile {
            id: tile_id,
            data: bitmap,
        })
    }

    /**
    Can other be flipped or rotated in a way that it aligns with self on a given side?
    */
    pub fn aligns(&self, other: &mut Tile, border: &Border) -> bool {
        for _rotation in 0..4 {
            other.rotate(90);
            for _h_flip in 0..2 {
                other.flip(Border::TOP);
                for _v_flip in 0..2 {
                    other.flip(Border::LEFT);
                    if self.aligns_directly(other, border) {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn aligns_directly(&self, other: &Tile, border: &Border) -> bool {
        match border {
            Border::TOP => self.data[..10] == other.data[90..],
            Border::RIGHT => {
                self.data.iter().enumerate().filter(|&(k, _)| k % 10 == 9).map(|(_, &v)| v).collect::<Vec<bool>>()
                    == other.data.iter().enumerate().filter(|&(k, _)| k % 10 == 0).map(|(_, &v)| v).collect::<Vec<bool>>()
            }
            Border::BOTTOM => self.data[90..] == other.data[..10],
            Border::LEFT => {
                self.data.iter().enumerate().filter(|&(k, _)| k % 10 == 0).map(|(_, &v)| v).collect::<Vec<bool>>()
                    == other.data.iter().enumerate().filter(|&(k, _)| k % 10 == 9).map(|(_, &v)| v).collect::<Vec<bool>>()
            }
        }
    }

    fn rotate(&mut self, degrees: i16) {
        let cloned = self.data;
        let degrees = (degrees + 360) % 360;
        match degrees {
            0 => {}
            90 => {
                self.data = <[bool; 100]>::try_from(rotate(&cloned)).unwrap();
                /*for (idx, &bit) in cloned.iter().enumerate() {
                    let row = idx / 10;
                    let col = idx % 10;
                    self.data[col * 10 + (9 - row)] = bit;
                }*/
            }
            180 => {
                self.data = <[bool; 100]>::try_from(rotate(&rotate(&cloned))).unwrap();
            }
            270 => {
                self.data = <[bool; 100]>::try_from(rotate(&rotate(&rotate(&cloned)))).unwrap();
            }
            _ => {}
        }
    }

    fn flip(&mut self, border: Border) {
        self.data = <[bool; 100]>::try_from(flip(&self.data, border)).unwrap();
    }
}

pub fn rotate<T: Copy>(data: &[T]) -> Vec<T> {
    let width = (data.len() as f64).sqrt() as usize;
    let mut res = vec![data[0]; data.len()];
    for (idx, &bit) in data.iter().enumerate() {
        let row = idx / width;
        let col = idx % width;
        res[col * width + (width - 1 - row)] = bit;
    }
    res
}

pub fn flip<T: Copy>(data: &[T], border: Border) -> Vec<T> {
    let width = (data.len() as f64).sqrt() as usize;
    let mut res = vec![data[0]; data.len()];
    match border {
        Border::TOP | Border::BOTTOM => {
            for (idx, &bit) in data.iter().enumerate() {
                let row = idx / width;
                let col = idx % width;
                res[(width - 1 - row) * width + col] = bit;
            }
        }
        Border::RIGHT | Border::LEFT => {
            for (idx, &bit) in data.iter().enumerate() {
                let row = idx / width;
                let col = idx % width;
                res[row * width + (width - 1 - col)] = bit;
            }
        }
    }
    res
}

pub fn get_tiles(input: &str) -> Option<Vec<Tile>> {
    let mut res: Vec<Tile> = Vec::new();
    for tile in input.trim().split("\n\n") {
        res.push(Tile::new(tile)?);
    }
    Some(res)
}

pub fn get_sorted_tiles(mut tiles: Vec<Tile>) -> Option<Vec<Tile>> {
    let width = (tiles.len() as f64).sqrt() as usize;
    sort_tiles(&mut Vec::new(), &mut tiles, width)
}

pub fn sort_tiles(found: &mut Vec<Tile>, available: &mut Vec<Tile>, width: usize) -> Option<Vec<Tile>> {
    //println!("Entry. Found: {}, available: {}", found.len(), available.len());
    if available.len() == 0 { return Some(found.to_vec()); }
    if found.len() == 0 {
        for i in 0..available.len() {
            //println!("i: {}, Found: {}, available: {}", i, found.len(), available.len());
            let mut tile = available.remove(i);
            for _rotation in 0..4 {
                tile.rotate(90);
                for _hflip in 0..2 {
                    tile.flip(Border::BOTTOM);
                    for _vflip in 0..2 {
                        tile.flip(Border::LEFT);

                        found.push(tile);
                        match sort_tiles(found, available, width) {
                            None => {
                                // undo and continue
                                found.pop();
                            }
                            Some(res) => {
                                return Some(res);
                            }
                        }
                    }
                }
            }
            available.insert(i, tile);
        }
    } else {
        for i in 0..available.len() {
            //println!("i: {}, Found: {}, available: {}", i, found.len(), available.len());
            let mut tile = available.remove(i);
            if found.len() % width != 0 {
                // has left neighbour
                if found.iter().last()?.aligns(&mut tile, &Border::RIGHT) {
                    if found.len() / width == 0 ||
                        found.get(found.len() - width)?.aligns_directly(&tile, &Border::BOTTOM) {
                        // has no top neighbour or aligns directly
                        found.push(tile);
                        match sort_tiles(found, available, width) {
                            None => {
                                // undo and continue
                                found.pop();
                            }
                            Some(res) => {
                                return Some(res);
                            }
                        }
                    }
                }
            } else {
                // must have top neighbour since it's not the first
                if found.get(found.len() - width)?.aligns(&mut tile, &Border::BOTTOM) {
                    found.push(tile);
                    match sort_tiles(found, available, width) {
                        None => {
                            // undo and continue
                            found.pop();
                        }
                        Some(res) => {
                            return Some(res);
                        }
                    }
                }
            }
            available.insert(i, tile);
        }
    }
    None
}

pub fn get_product_of_corners(input: &str) -> Option<usize> {
    let mut tiles = get_tiles(input)?;
    let width = (tiles.len() as f64).sqrt() as usize;
    let tiles = sort_tiles(&mut Vec::new(), &mut tiles, width)?;
    Some((tiles.get(0)?.id as usize)
        * (tiles.get(width - 1)?.id as usize)
        * (tiles.get(tiles.len() - 1)?.id as usize)
        * (tiles.get(tiles.len() - width)?.id as usize))
}

fn get_image(tiles: &[Tile]) -> Vec<bool> {
    let width = (tiles.len() as f64).sqrt() as usize;
    let mut image: Vec<bool> = vec![false; width * 8 * width * 8];

    for (idx, tile) in tiles.iter().enumerate() {
        let tile_row = idx / width;
        let tile_col = idx % width;
        for (bit_idx, &val) in tile.data.iter().enumerate().filter(|&(k, _)| {
            k > 9 && k < 90 && k % 10 != 0 && k % 10 != 9
        }) {
            let row = bit_idx / 10;
            let col = bit_idx % 10;
            image[(tile_row * 8 + row - 1) * width * 8 + tile_col * 8 + col - 1] = val;
        }
    }
    image
}

pub fn get_water_roughness(tiles: Vec<Tile>) -> usize {
    let width = (tiles.len() as f64).sqrt() as usize;
    let sea_monster = get_sea_monster(width);
    let image = get_image(&tiles);
    let monsters = count_sea_monsters(tiles);
    image.iter().filter(|&v| *v).count() - monsters * sea_monster.len()
}

pub fn count_sea_monsters(tiles: Vec<Tile>) -> usize {
    let width = (tiles.len() as f64).sqrt() as usize;
    println!("{}", width);
    let sea_monster = get_sea_monster(width);
    let mut image = get_image(&tiles);
    let mut max_count = 0;
    for _ in 0..4 {
        for _ in 0..2 {
            for _ in 0..2 {
                let mut count = 0;
                for y in 0..(width * 8 - 3) {
                    for x in 0..(width * 8 - 20) {
                        if sea_monster.iter().all(|v| image[v + y * width * 8 + x]) {
                            count += 1;
                        }
                    }
                }
                println!("{}", count);
                if count > max_count {
                    max_count = count;
                }
                image = flip(&image, Border::LEFT);
            }
            image = flip(&image, Border::TOP);
        }
        image = rotate(&image);
    }
    max_count
}

fn get_sea_monster(width: usize) -> Vec<usize> {
    let sea_monster = r"                  #
#    ##    ##    ###
 #  #  #  #  #  #   ";
    let mut monster = Vec::new();
    for (lid, line) in sea_monster.lines().enumerate() {
        for (cid, chr) in line.chars().enumerate() {
            if chr == '#' {
                monster.push(lid * width * 8 + cid);
            }
        }
    }
    monster
}

#[cfg(test)]
mod tests {
    use crate::day20::{Tile, get_tiles, sort_tiles, get_product_of_corners, count_sea_monsters, get_water_roughness, get_sorted_tiles};
    use crate::day20::Border;

    #[test]
    fn test_get_tile() {
        let input = r"Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###";
        let expected = Tile {
            id: 2311,
            data: [false, false, true, true, false, true, false, false, true, false,
                true, true, false, false, true, false, false, false, false, false,
                true, false, false, false, true, true, false, false, true, false,
                true, true, true, true, false, true, false, false, false, true,
                true, true, false, true, true, false, true, true, true, false,
                true, true, false, false, false, true, false, true, true, true,
                false, true, false, true, false, true, false, false, true, true,
                false, false, true, false, false, false, false, true, false, false,
                true, true, true, false, false, false, true, false, true, false,
                false, false, true, true, true, false, false, true, true, true],
        };
        assert_eq!(Some(expected), Tile::new(input));
    }

    #[test]
    fn test_rotate() {
        let mut input_data = [false; 100];
        for i in 0..10 {
            input_data[10 * i + i] = true;
        }
        let mut expected_data = [false; 100];
        for i in 0..10 {
            expected_data[10 * (i + 1) - i - 1] = true;
        }
        let mut rotated = Tile {
            id: 0,
            data: input_data,
        };
        rotated.rotate(90);
        assert_eq!(Tile { id: 0, data: expected_data }, rotated);
        rotated.rotate(90);
        assert_eq!(Tile { id: 0, data: input_data }, rotated);
        rotated.rotate(90);
        assert_eq!(Tile { id: 0, data: expected_data }, rotated);
        rotated.rotate(90);
        assert_eq!(Tile { id: 0, data: input_data }, rotated);
        rotated.rotate(90);
        assert_ne!(Tile { id: 0, data: input_data }, rotated);
        rotated.rotate(-90);
        assert_eq!(Tile { id: 0, data: input_data }, rotated);
        rotated.rotate(180);
        assert_eq!(Tile { id: 0, data: input_data }, rotated);
    }

    #[test]
    fn test_flip() {
        let mut input_data = [false; 100];
        for i in 0..10 {
            input_data[10 * i + i] = true;
        }
        let mut expected_data = [false; 100];
        for i in 0..10 {
            expected_data[10 * i + (9 - i)] = true;
        }
        let mut flipped = Tile {
            id: 0,
            data: input_data,
        };
        flipped.flip(Border::LEFT);
        assert_eq!(Tile { id: 0, data: expected_data }, flipped);
        flipped.flip(Border::LEFT);
        assert_eq!(Tile { id: 0, data: input_data }, flipped);
        flipped.flip(Border::TOP);
        assert_eq!(Tile { id: 0, data: expected_data }, flipped);
        flipped.flip(Border::TOP);
        assert_eq!(Tile { id: 0, data: input_data }, flipped);
    }

    #[test]
    fn test_align() {
        let mut t2311 = Tile::new(r"Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###").unwrap();
        let t1951 = Tile::new(r"Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..").unwrap();
        assert!(t1951.aligns(&mut t2311, &Border::RIGHT));
    }

    #[test]
    fn test_get_tiles() {
        let input = std::fs::read_to_string("resources/day20_example.txt").unwrap();
        let tiles = get_tiles(&input).unwrap();
        assert_eq!(9, tiles.len());
    }

    #[test]
    fn test_get_corners() {
        let input = std::fs::read_to_string("resources/day20_example.txt").unwrap();
        assert_eq!(20899048083289, get_product_of_corners(&input).unwrap());
    }

    #[test]
    #[ignore] // takes a long time (quicker to run after cargo build --release)
    fn test_part1() {
        let input = std::fs::read_to_string("resources/day20.txt").unwrap();
        println!("{:?}", get_product_of_corners(&input));
    }

    #[test]
    fn test_get_seamonsters() {
        let input = std::fs::read_to_string("resources/day20_example.txt").unwrap();
        let tiles = sort_tiles(&mut Vec::new(), &mut get_tiles(&input).unwrap(), 3).unwrap();
        assert_eq!(2, count_sea_monsters(tiles));
    }

    #[test]
    fn test_water_roughness() {
        let input = std::fs::read_to_string("resources/day20_example.txt").unwrap();
        let tiles = get_sorted_tiles(get_tiles(&input).unwrap()).unwrap();
        assert_eq!(273, get_water_roughness(tiles));
    }

    #[test]
    #[ignore] // takes a long time (quicker to run after cargo build --release)
    fn test_part2() {
        let input = std::fs::read_to_string("resources/day20.txt").unwrap();
        let tiles = get_sorted_tiles(get_tiles(&input).unwrap()).unwrap();
        println!("{:?}", get_water_roughness(tiles));
    }


}