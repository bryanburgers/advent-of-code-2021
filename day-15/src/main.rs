use std::{
    collections::VecDeque,
    ops::{Index, IndexMut},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("input.txt");
    let grid = parse_input(input)?;
    let costs = grid.traverse_from(Coordinate::new(0, 0));
    let cost = costs[Coordinate::new(grid.width - 1, grid.height - 1)];
    println!("{}", cost);
    Ok(())
}

#[derive(Debug)]
struct Grid<T> {
    width: usize,
    height: usize,
    values: Vec<T>,
}

fn parse_input(s: &str) -> Result<Grid<u8>, String> {
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

    let height = rows.len();
    let mut values = Vec::new();
    for mut row in rows {
        values.append(&mut row);
    }

    Ok(Grid {
        width,
        height,
        values,
    })
}

impl<T> Index<Coordinate> for Grid<T> {
    type Output = T;

    fn index(&self, coordinate: Coordinate) -> &Self::Output {
        if coordinate.x >= self.width {
            panic!("Out of bounds: x");
        }
        if coordinate.y >= self.height {
            panic!("Out of bounds: y");
        }
        &self.values[coordinate.y * self.width + coordinate.x]
    }
}

impl<T> IndexMut<Coordinate> for Grid<T> {
    fn index_mut(&mut self, coordinate: Coordinate) -> &mut Self::Output {
        if coordinate.x >= self.width {
            panic!("Out of bounds: x");
        }
        if coordinate.y >= self.height {
            panic!("Out of bounds: y");
        }
        &mut self.values[coordinate.y * self.width + coordinate.x]
    }
}

impl<T> Grid<T> {
    pub fn neighbors<'a>(&'a self, coordinate: Coordinate) -> NeighborsIterator<'a, T> {
        NeighborsIterator::new(self, coordinate)
    }

    pub fn all<'a>(&'a self) -> AllIterator<'a, T> {
        AllIterator::new(self)
    }
}

impl Grid<u8> {
    pub fn traverse_from(&self, start: Coordinate) -> Grid<usize> {
        let mut values = Vec::new();
        values.resize(self.width * self.height, usize::MAX);
        let mut traversed_grid = Grid {
            width: self.width,
            height: self.height,
            values,
        };

        traversed_grid[start] = 0;

        let mut unvisited: VecDeque<Coordinate> = self.all().filter(|c| c != &start).collect();
        unvisited.push_front(start);

        while let Some(current) = unvisited.pop_front() {
            let current_cost = traversed_grid[current];
            for neighbor_coordinate in self.neighbors(current) {
                let neighbor_cost = self[neighbor_coordinate] as usize;
                let total_cost = current_cost + neighbor_cost;
                let previous_best_cost = traversed_grid[neighbor_coordinate];
                if total_cost < previous_best_cost {
                    traversed_grid[neighbor_coordinate] = total_cost;
                    if let Some(index) = unvisited
                        .iter()
                        .enumerate()
                        .find(|(_idx, c)| **c == neighbor_coordinate)
                        .map(|x| x.0)
                    {
                        unvisited.remove(index);
                        match unvisited.binary_search_by(|coordinate| {
                            traversed_grid[*coordinate].cmp(&total_cost)
                        }) {
                            Ok(idx) => unvisited.insert(idx, neighbor_coordinate),
                            Err(idx) => unvisited.insert(idx, neighbor_coordinate),
                        }
                    }
                }
            }
        }

        traversed_grid
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

struct NeighborsIterator<'a, T> {
    cave: &'a Grid<T>,
    directions: VecDeque<Direction>,
    coordinate: Coordinate,
}

impl<'a, T> NeighborsIterator<'a, T> {
    pub fn new(cave: &'a Grid<T>, coordinate: Coordinate) -> Self {
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

impl<'a, T> Iterator for NeighborsIterator<'a, T> {
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

struct AllIterator<'a, T> {
    cave: &'a Grid<T>,
    coordinate: Coordinate,
}

impl<'a, T> AllIterator<'a, T> {
    pub fn new(cave: &'a Grid<T>) -> Self {
        AllIterator {
            cave,
            coordinate: Coordinate::new(0, 0),
        }
    }
}

impl<'a, T> Iterator for AllIterator<'a, T> {
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
    fn test_travserse() {
        let subject = parse_input(include_str!("example.txt")).unwrap();
        let costs = subject.traverse_from(Coordinate::new(0, 0));
        println!("{:?}", costs);
    }
}
