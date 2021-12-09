use std::{
    collections::{HashMap, VecDeque},
    ops::Index,
    str::FromStr,
};

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    let cave = input.parse::<Cave>().unwrap();
    let result = solve_a(&cave);
    println!("{}", result);

    let result = solve_b(&cave);
    println!("{}", result);

    Ok(())
}

fn solve_a(cave: &Cave) -> usize {
    let mut sum = 0;
    for coordinate in cave.all() {
        if cave.is_lowpoint(coordinate) {
            sum += cave.risk_level(coordinate);
        }
    }
    sum
}

fn solve_b(cave: &Cave) -> usize {
    let mut basins: HashMap<Coordinate, usize> = HashMap::new();

    for coordinate in cave.all() {
        if let Some(basin) = cave.basin_point(coordinate) {
            basins.entry(basin).and_modify(|v| *v += 1).or_insert(1);
        }
    }

    let mut basins: Vec<usize> = basins.into_values().collect();
    basins.sort_by(|a, b| a.cmp(b).reverse());

    assert!(basins.len() > 3);

    basins[0] * basins[1] * basins[2]
}

struct Cave {
    width: usize,
    height: usize,
    depths: Vec<Vec<u8>>,
}

impl FromStr for Cave {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn line_to_depths(s: &str) -> Result<Vec<u8>, String> {
            s.char_indices()
                .map(|(idx, _)| {
                    let v = &s[idx..][..1];
                    v.parse::<u8>().map_err(|err| err.to_string())
                })
                .collect()
        }

        let mut rows = Vec::new();
        let mut lines = s.lines();
        let first_line = lines.next().ok_or("No first line")?;
        let first = line_to_depths(first_line)?;
        let width = first.len();
        rows.push(first);

        for (idx, line) in lines.enumerate() {
            let line = line_to_depths(line)?;
            if line.len() != width {
                return Err(format!(
                    "Line {} was length {} but line 0 was length {}",
                    idx + 1,
                    line.len(),
                    width
                ));
            }
            rows.push(line)
        }

        Ok(Cave {
            width,
            height: rows.len(),
            depths: rows,
        })
    }
}

impl Index<Coordinate> for Cave {
    type Output = u8;

    fn index(&self, coordinate: Coordinate) -> &Self::Output {
        if coordinate.x >= self.width {
            panic!("Out of bounds: x");
        }
        if coordinate.y >= self.height {
            panic!("Out of bounds: y");
        }
        &self.depths[coordinate.y][coordinate.x]
    }
}

impl Cave {
    pub fn neighbors(&self, coordinate: Coordinate) -> NeighborsIterator {
        NeighborsIterator::new(self, coordinate)
    }

    pub fn all(&self) -> AllIterator {
        AllIterator::new(self)
    }

    pub fn is_lowpoint(&self, coordinate: Coordinate) -> bool {
        let val = self[coordinate];
        !self
            .neighbors(coordinate)
            .any(|neighbor| self[neighbor] <= val)
    }

    pub fn risk_level(&self, coordinate: Coordinate) -> usize {
        self[coordinate] as usize + 1
    }

    pub fn basin_point(&self, coordinate: Coordinate) -> Option<Coordinate> {
        fn flow_to(cave: &Cave, coordinate: Coordinate) -> Option<Coordinate> {
            for neighbor in cave.neighbors(coordinate) {
                if cave[neighbor] < cave[coordinate] {
                    return Some(neighbor);
                }
            }
            None
        }

        if self[coordinate] == 9 {
            return None;
        }

        let mut coordinate = coordinate;
        loop {
            if let Some(c) = flow_to(self, coordinate) {
                coordinate = c;
                continue;
            } else {
                return Some(coordinate);
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct NeighborsIterator<'a> {
    cave: &'a Cave,
    directions: VecDeque<Direction>,
    coordinate: Coordinate,
}

impl<'a> NeighborsIterator<'a> {
    pub fn new(cave: &'a Cave, coordinate: Coordinate) -> Self {
        NeighborsIterator {
            cave,
            directions: VecDeque::from([
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ]),
            coordinate,
        }
    }
}

impl<'a> Iterator for NeighborsIterator<'a> {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(direction) = self.directions.pop_front() {
                let option_coordinate = match direction {
                    Direction::Up if self.coordinate.y > 0 => Some(Coordinate {
                        x: self.coordinate.x,
                        y: self.coordinate.y - 1,
                    }),
                    Direction::Down if self.coordinate.y + 1 < self.cave.height => {
                        Some(Coordinate {
                            x: self.coordinate.x,
                            y: self.coordinate.y + 1,
                        })
                    }
                    Direction::Left if self.coordinate.x > 0 => Some(Coordinate {
                        x: self.coordinate.x - 1,
                        y: self.coordinate.y,
                    }),
                    Direction::Right if self.coordinate.x + 1 < self.cave.width => {
                        Some(Coordinate {
                            x: self.coordinate.x + 1,
                            y: self.coordinate.y,
                        })
                    }
                    _ => None,
                };
                if let Some(coordinate) = option_coordinate {
                    return Some(coordinate);
                }
            } else {
                return None;
            }
        }
    }
}

struct AllIterator<'a> {
    cave: &'a Cave,
    coordinate: Coordinate,
}

impl<'a> AllIterator<'a> {
    pub fn new(cave: &'a Cave) -> Self {
        AllIterator {
            cave,
            coordinate: Coordinate::new(0, 0),
        }
    }
}

impl<'a> Iterator for AllIterator<'a> {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        if self.coordinate.x < self.cave.width && self.coordinate.y < self.cave.height {
            let ret = self.coordinate;
            self.coordinate.x += 1;
            Some(ret)
        } else if self.coordinate.y + 1 < self.cave.height {
            self.coordinate.y += 1;
            self.coordinate.x = 0;
            let ret = self.coordinate;
            self.coordinate.x += 1;
            Some(ret)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coordinates() {
        let cave = Cave {
            width: 3,
            height: 2,
            depths: vec![vec![1, 2, 3], vec![4, 5, 6]],
        };

        assert_eq!(cave[Coordinate::new(0, 0)], 1);
        assert_eq!(cave[Coordinate::new(2, 1)], 6);

        let coordinates = cave.neighbors(Coordinate::new(1, 0)).collect::<Vec<_>>();
        assert!(coordinates.contains(&Coordinate::new(0, 0)));
        assert!(coordinates.contains(&Coordinate::new(2, 0)));
        assert!(coordinates.contains(&Coordinate::new(1, 1)));

        let coordinates = cave.all().collect::<Vec<_>>();
        assert!(coordinates.contains(&Coordinate::new(0, 0)));
        assert!(coordinates.contains(&Coordinate::new(1, 0)));
        assert!(coordinates.contains(&Coordinate::new(2, 0)));
        assert!(coordinates.contains(&Coordinate::new(0, 1)));
        assert!(coordinates.contains(&Coordinate::new(1, 1)));
        assert!(coordinates.contains(&Coordinate::new(2, 1)));
        assert_eq!(coordinates.len(), cave.width * cave.height);
    }

    #[test]
    fn test_solve_a() {
        let input = include_str!("example.txt");
        let cave = input.parse::<Cave>().unwrap();
        let result = solve_a(&cave);
        assert_eq!(result, 15);
    }

    #[test]
    fn lower_than_not_lower_or_equal_to() {
        let input = "99\n90";
        let cave = input.parse::<Cave>().unwrap();
        assert!(!cave.is_lowpoint(Coordinate::new(0, 0)));
    }

    #[test]
    fn test_basin_point() {
        let input = include_str!("example.txt");
        let cave = input.parse::<Cave>().unwrap();

        // The lowpoint in the top right is its own basin point
        assert_eq!(
            cave.basin_point(Coordinate::new(1, 0)),
            Some(Coordinate::new(1, 0))
        );
        // The other two points that make up the basin have the sam basin point
        assert_eq!(
            cave.basin_point(Coordinate::new(0, 0)),
            Some(Coordinate::new(1, 0))
        );
        assert_eq!(
            cave.basin_point(Coordinate::new(0, 1)),
            Some(Coordinate::new(1, 0))
        );

        // The 9 near that basin does not have a basin point
        assert_eq!(cave.basin_point(Coordinate::new(2, 0)), None)
    }

    #[test]
    fn test_solve_b() {
        let input = include_str!("example.txt");
        let cave = input.parse::<Cave>().unwrap();
        let result = solve_b(&cave);
        assert_eq!(result, 1134);
    }
}
