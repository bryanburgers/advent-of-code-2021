use std::{collections::HashSet, fmt::Debug, str::FromStr};

fn main() {
    let (alg, grid) = parse(include_str!("input.txt")).unwrap();
    let result = solve(&alg, &grid, 2);
    println!("{}", result);
    let result = solve(&alg, &grid, 50);
    println!("{}", result);
}

fn solve(alg: &ImageEnhancementAlgorithm, grid: &Grid, n: usize) -> usize {
    let mut next_grid = alg.step(grid);
    for _ in 1..n {
        next_grid = alg.step(&next_grid);
    }
    next_grid.lit_pixels.len()
}

fn parse(input: &str) -> Result<(ImageEnhancementAlgorithm, Grid), &'static str> {
    let (a, b) = input.split_once("\n\n").ok_or("Invalid input")?;
    let a = a.parse()?;
    let b = b.parse()?;
    Ok((a, b))
}

struct ImageEnhancementAlgorithm {
    data: Box<[bool; 512]>,
}

impl ImageEnhancementAlgorithm {
    pub fn calculate_offset(&self, coordinate: Coordinate, grid: &Grid) -> u16 {
        let mut offset = 0;
        for neighbor in coordinate.neighbors() {
            offset <<= 1;
            if grid.is_set(neighbor) {
                offset |= 1;
            }
        }
        offset
    }

    pub fn calculate_infinite_default_offset(&self, grid: &Grid) -> u16 {
        if grid.infinite_default {
            511
        } else {
            0
        }
    }

    pub fn step(&self, grid: &Grid) -> Grid {
        let mut new_grid = Grid::empty();
        let infinite_default_offset = self.calculate_infinite_default_offset(grid);
        let infinite_default_value = self.data[usize::from(infinite_default_offset)];
        new_grid.set_infinite_default(infinite_default_value);
        for x in (grid.min_x - 1)..=(grid.max_x + 1) {
            for y in (grid.min_y - 1)..=(grid.max_y + 1) {
                let coordinate = Coordinate { x, y };
                let offset = self.calculate_offset(coordinate, grid);
                let is_set = self.data[usize::from(offset)];
                if is_set {
                    new_grid.set(coordinate);
                }
            }
        }
        new_grid
    }
}

impl FromStr for ImageEnhancementAlgorithm {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 512 {
            Err("Must be exactly 512 bytes")
        } else {
            let mut data = Box::new([false; 512]);
            let s_bytes = s.as_bytes();
            for idx in 0..512 {
                if s_bytes[idx] == b'#' {
                    data[idx] = true;
                }
            }
            Ok(Self { data })
        }
    }
}

struct Grid {
    infinite_default: bool,
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
    lit_pixels: HashSet<Coordinate>,
}

impl Grid {
    pub fn empty() -> Grid {
        Self {
            infinite_default: false,
            min_x: 0,
            max_x: 0,
            min_y: 0,
            max_y: 0,
            lit_pixels: HashSet::new(),
        }
    }

    pub fn set_infinite_default(&mut self, v: bool) {
        self.infinite_default = v;
    }

    pub fn set(&mut self, coordinate: Coordinate) {
        self.min_x = std::cmp::min(self.min_x, coordinate.x);
        self.max_x = std::cmp::max(self.max_x, coordinate.x);
        self.min_y = std::cmp::min(self.min_y, coordinate.y);
        self.max_y = std::cmp::max(self.max_y, coordinate.y);
        self.lit_pixels.insert(coordinate);
    }

    pub fn is_set(&self, coordinate: Coordinate) -> bool {
        if coordinate.x < self.min_x
            || coordinate.y < self.min_y
            || coordinate.x > self.max_x
            || coordinate.y > self.max_y
        {
            self.infinite_default
        } else {
            self.lit_pixels.contains(&coordinate)
        }
    }
}

impl FromStr for Grid {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = Grid::empty();
        for (y, line) in s.lines().enumerate() {
            for (x, byte) in line.bytes().enumerate() {
                if byte == b'#' {
                    let coordinate = Coordinate {
                        x: x.try_into().map_err(|_| "Out of range")?,
                        y: y.try_into().map_err(|_| "Out of range")?,
                    };
                    grid.set(coordinate);
                } else if byte == b'.' {
                    // Do nothing.
                } else {
                    return Err("Invalid character");
                }
            }
        }

        Ok(grid)
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in (self.min_y - 1)..=(self.max_y + 1) {
            for x in (self.min_x - 1)..=(self.max_x + 1) {
                let coordinate = Coordinate { x, y };
                if self.is_set(coordinate) {
                    f.write_str("#")?;
                } else {
                    f.write_str(".")?;
                }
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Coordinate {
    x: i64,
    y: i64,
}

impl Coordinate {
    pub fn offset<T: Into<i64>>(self, x: T, y: T) -> Self {
        Self {
            x: self.x + x.into(),
            y: self.y + y.into(),
        }
    }

    pub fn neighbors(self) -> NeighborsIterator {
        NeighborsIterator::new(self)
    }
}

struct NeighborsIterator {
    coordinate: Coordinate,
    offset_index: u8,
}

impl NeighborsIterator {
    const OFFSETS: [(i8, i8); 9] = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (0, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    fn new(coordinate: Coordinate) -> Self {
        Self {
            coordinate,
            offset_index: 0,
        }
    }
}

impl Iterator for NeighborsIterator {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset_index >= 9 {
            None
        } else {
            let (offset_x, offset_y): (i8, i8) = Self::OFFSETS[usize::from(self.offset_index)];
            self.offset_index += 1;
            Some(self.coordinate.offset(offset_x, offset_y))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_a() {
        let (alg, grid) = parse(include_str!("example.txt")).unwrap();
        let result = solve(&alg, &grid, 2);
        assert_eq!(result, 35);
    }

    #[test]
    fn test_solve_b() {
        let (alg, grid) = parse(include_str!("example.txt")).unwrap();
        let result = solve(&alg, &grid, 50);
        assert_eq!(result, 3351);
    }
}
