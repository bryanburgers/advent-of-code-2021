use std::{collections::VecDeque, fmt::Debug};

fn main() {
    let mut input = OctopusPod::from([
        [4, 4, 7, 2, 5, 6, 2, 2, 6, 4],
        [8, 6, 3, 1, 5, 1, 7, 8, 2, 7],
        [7, 2, 3, 2, 1, 4, 4, 1, 4, 6],
        [2, 4, 4, 7, 1, 6, 3, 8, 2, 4],
        [1, 2, 3, 5, 2, 7, 2, 6, 7, 1],
        [5, 1, 3, 3, 5, 2, 7, 1, 4, 6],
        [6, 5, 1, 1, 3, 7, 2, 4, 1, 7],
        [3, 8, 4, 1, 8, 4, 1, 6, 1, 4],
        [8, 6, 2, 1, 3, 6, 8, 7, 8, 2],
        [3, 2, 4, 6, 3, 3, 6, 6, 7, 7],
    ]);
    let input_b = input.clone();
    let flashes = input.steps(100);
    println!("{}", flashes);
    let steps = solve_b(input_b);
    println!("{}", steps);
}

fn solve_b<const C: usize>(mut input: OctopusPod<C>) -> usize {
    for i in 1.. {
        let val = input.step();
        if val == C * C {
            return i;
        }
    }
    0
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Octopus {
    PowerLevel(u8),
    HasFlashed,
}

impl Octopus {
    pub const fn from_u8(val: u8) -> Self {
        if val > 9 {
            Self::HasFlashed
        } else {
            Self::PowerLevel(val)
        }
    }

    pub const fn has_flashed(&self) -> bool {
        match self {
            Self::HasFlashed => true,
            _ => false,
        }
    }

    pub fn next_round(&mut self) {
        if self.has_flashed() {
            *self = Self::PowerLevel(0)
        }
    }

    pub fn inc(&mut self) -> bool {
        match self {
            Self::PowerLevel(v) if *v >= 9 => { *self = Self::HasFlashed; true },
            Self::PowerLevel(v) => { *self = Self::PowerLevel(*v + 1); false },
            Self::HasFlashed => false,
        }
    }
}

impl Debug for Octopus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PowerLevel(arg0) => write!(f, "{}", arg0),
            Self::HasFlashed => write!(f, "*"),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct OctopusPod<const C: usize> {
    octopuses: [[Octopus; C]; C],
}

impl<const C: usize> OctopusPod<C> {
    fn get_mut(&mut self, coordinate: Coordinate<C>) -> &mut Octopus {
        &mut self.octopuses[coordinate.y][coordinate.x]
    }

    fn step(&mut self) -> usize {
        let mut work_queue: VecDeque<Coordinate<C>> = Coordinate::all().collect();
        let mut flashes = 0;

        while let Some(coordinate) = work_queue.pop_front() {
            let octopus = self.get_mut(coordinate);
            let did_flash = octopus.inc();
            if did_flash {
                flashes += 1;
                for neighbor in coordinate.neighbors() {
                    work_queue.push_back(neighbor);
                }
            }
        }

        for coordinate in Coordinate::all() {
            let octopus = self.get_mut(coordinate);
            octopus.next_round();
        }

        flashes
    }

    fn steps(&mut self, steps: usize) -> usize {
        let mut sum = 0;
        for _ in 0..steps {
            sum += self.step();
        }
        sum
    }
}

impl<const C: usize> From<[[u8; C]; C]> for OctopusPod<C> {
    fn from(val: [[u8; C]; C]) -> Self {
        let mut octopuses = [[Octopus::PowerLevel(0); C]; C];
        for x in 0..C {
            for y in 0..C {
                octopuses[y][x] = Octopus::from_u8(val[y][x]);
            }
        }
        Self {
            octopuses
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct Coordinate<const C: usize> {
    x: usize,
    y: usize,
}

impl<const C: usize> Coordinate<C> {
    fn new_unchecked(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn left(&self) -> Option<Self> {
        if self.x > 0 {
            Some(Self { x: self.x - 1, y: self.y })
        }
        else {
            None
        }
    }

    fn right(&self) -> Option<Self> {
        if self.x + 1 < C {
            Some(Self { x: self.x + 1, y: self.y })
        }
        else {
            None
        }
    }

    fn up(&self) -> Option<Self> {
        if self.y > 0 {
            Some(Self { x: self.x, y: self.y - 1 })
        }
        else {
            None
        }
    }

    fn down(&self) -> Option<Self> {
        if self.y + 1 < C {
            Some(Self { x: self.x, y: self.y + 1 })
        }
        else {
            None
        }
    }

    fn neighbors(self) -> NeighborsIterator<C> {
        NeighborsIterator::new(self)
    }

    fn all() -> AllIterator<C> {
        AllIterator::new()
    }
}

impl<const C: usize> Debug for Coordinate<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ {}, {} }}", self.x, self.y)
    }
}

struct NeighborsIterator<const C: usize> {
    coordinate: Coordinate<C>,
    remaining: Vec<IteratorDirection>,
}

enum IteratorDirection {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl<const C: usize> NeighborsIterator<C> {
    fn new(coordinate: Coordinate<C>) -> Self {
        let remaining = vec![
            IteratorDirection::Up,
            IteratorDirection::Down,
            IteratorDirection::Left,
            IteratorDirection::Right,
            IteratorDirection::UpLeft,
            IteratorDirection::UpRight,
            IteratorDirection::DownLeft,
            IteratorDirection::DownRight,
        ];
        Self {
            coordinate,
            remaining,
        }
    }
}

impl<const C: usize> Iterator for NeighborsIterator<C> {
    type Item = Coordinate<C>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(direction) = self.remaining.pop() {
            match direction {
                IteratorDirection::Up => {
                    if let Some(coordinate) = self.coordinate.up() {
                        return Some(coordinate);
                    }
                }
                IteratorDirection::Down => {
                    if let Some(coordinate) = self.coordinate.down() {
                        return Some(coordinate);
                    }
                }
                IteratorDirection::Left => {
                    if let Some(coordinate) = self.coordinate.left() {
                        return Some(coordinate);
                    }
                }
                IteratorDirection::Right => {
                    if let Some(coordinate) = self.coordinate.right() {
                        return Some(coordinate);
                    }
                }
                IteratorDirection::UpLeft => {
                    if let Some(coordinate) = self.coordinate.up().and_then(|coordinate| coordinate.left()) {
                        return Some(coordinate);
                    }
                }
                IteratorDirection::UpRight => {
                    if let Some(coordinate) = self.coordinate.up().and_then(|coordinate| coordinate.right()) {
                        return Some(coordinate);
                    }
                }
                IteratorDirection::DownLeft => {
                    if let Some(coordinate) = self.coordinate.down().and_then(|coordinate| coordinate.left()) {
                        return Some(coordinate);
                    }
                }
                IteratorDirection::DownRight => {
                    if let Some(coordinate) = self.coordinate.down().and_then(|coordinate| coordinate.right()) {
                        return Some(coordinate);
                    }
                }
            }
        }

        None
    }
}

struct AllIterator<const C: usize> {
    next_x: usize,
    next_y: usize,
}

impl<const C:usize> AllIterator<C> {
    fn new() -> Self {
        Self {
            next_x: 0,
            next_y: 0,
        }
    }
}

impl<const C: usize> Iterator for AllIterator<C> {
    type Item = Coordinate<C>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_y < C && self.next_x < C {
            let coordinate = Coordinate::new_unchecked(self.next_x, self.next_y);
            self.next_x += 1;
            Some(coordinate)
        }
        else if self.next_y + 1 < C {
            self.next_x = 0;
            self.next_y += 1;
            let coordinate = Coordinate::new_unchecked(self.next_x, self.next_y);
            self.next_x += 1;
            Some(coordinate)
        }
        else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_iterator() {
        let all_iterator = AllIterator::<10>::new();
        let items: Vec<_> = all_iterator.collect();
        assert_eq!(items.len(), 100);
    }

    #[test]
    fn neighbors_iterator() {
        let iterator: NeighborsIterator<10> = NeighborsIterator::new(Coordinate::new_unchecked(0, 0));
        let collected: Vec<_> = iterator.collect();
        assert_eq!(collected.len(), 3);

        let iterator: NeighborsIterator<10> = NeighborsIterator::new(Coordinate::new_unchecked(1, 1));
        let collected: Vec<_> = iterator.collect();
        assert_eq!(collected.len(), 8);

        let iterator: NeighborsIterator<10> = NeighborsIterator::new(Coordinate::new_unchecked(9, 9));
        let collected: Vec<_> = iterator.collect();
        assert_eq!(collected.len(), 3);

        let iterator: NeighborsIterator<10> = NeighborsIterator::new(Coordinate::new_unchecked(9, 5));
        let collected: Vec<_> = iterator.collect();
        assert_eq!(collected.len(), 5);
    }

    #[test]
    fn flash_test() {
        let mut subject = OctopusPod::from([
            [1, 1, 1, 1, 1],
            [1, 9, 9, 9, 1],
            [1, 9, 1, 9, 1],
            [1, 9, 9, 9, 1],
            [1, 1, 1, 1, 1],
        ]);
        let step_1 = OctopusPod::from([
            [3, 4, 5, 4, 3],
            [4, 0, 0, 0, 4],
            [5, 0, 0, 0, 5],
            [4, 0, 0, 0, 4],
            [3, 4, 5, 4, 3],
        ]);
        let step_2 = OctopusPod::from([
            [4, 5, 6, 5, 4],
            [5, 1, 1, 1, 5],
            [6, 1, 1, 1, 6],
            [5, 1, 1, 1, 5],
            [4, 5, 6, 5, 4],
        ]);

        subject.step();
        assert_eq!(subject, step_1);
        subject.step();
        assert_eq!(subject, step_2);
    }

    #[test]
    fn solve_part_a_2() {
        let mut subject = OctopusPod::from([
            [5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
            [2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
            [5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
            [6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
            [6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
            [4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
            [2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
            [6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
            [4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
            [5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
        ]);

        let step_2 = OctopusPod::from([
            [8, 8, 0, 7, 4, 7, 6, 5, 5, 5],
            [5, 0, 8, 9, 0, 8, 7, 0, 5, 4],
            [8, 5, 9, 7, 8, 8, 9, 6, 0, 8],
            [8, 4, 8, 5, 7, 6, 9, 6, 0, 0],
            [8, 7, 0, 0, 9, 0, 8, 8, 0, 0],
            [6, 6, 0, 0, 0, 8, 8, 9, 8, 9],
            [6, 8, 0, 0, 0, 0, 5, 9, 4, 3],
            [0, 0, 0, 0, 0, 0, 7, 4, 5, 6],
            [9, 0, 0, 0, 0, 0, 0, 8, 7, 6],
            [8, 7, 0, 0, 0, 0, 6, 8, 4, 8],
        ]);

        let flashes = subject.step();
        assert_eq!(flashes, 0);
        let flashes = subject.step();
        assert_eq!(subject, step_2);
        assert_eq!(flashes, 35);
    }

    #[test]
    fn solve_part_a_10() {
        let mut subject = OctopusPod::from([
            [5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
            [2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
            [5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
            [6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
            [6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
            [4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
            [2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
            [6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
            [4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
            [5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
        ]);

        let step_10 = OctopusPod::from([
            [0, 4, 8, 1, 1, 1, 2, 9, 7, 6],
            [0, 0, 3, 1, 1, 1, 2, 0, 0, 9],
            [0, 0, 4, 1, 1, 1, 2, 5, 0, 4],
            [0, 0, 8, 1, 1, 1, 1, 4, 0, 6],
            [0, 0, 9, 9, 1, 1, 1, 3, 0, 6],
            [0, 0, 9, 3, 5, 1, 1, 2, 3, 3],
            [0, 4, 4, 2, 3, 6, 1, 1, 3, 0],
            [5, 5, 3, 2, 2, 5, 2, 3, 5, 0],
            [0, 5, 3, 2, 2, 5, 0, 6, 0, 0],
            [0, 0, 3, 2, 2, 4, 0, 0, 0, 0],
        ]);

        let flashes = subject.steps(10);
        assert_eq!(flashes, 204);
        assert_eq!(subject, step_10);
    }

    #[test]
    fn solve_part_a_100() {
        let mut subject = OctopusPod::from([
            [5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
            [2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
            [5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
            [6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
            [6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
            [4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
            [2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
            [6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
            [4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
            [5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
        ]);

        let step_10 = OctopusPod::from([
            [0, 3, 9, 7, 6, 6, 6, 8, 6, 6],
            [0, 7, 4, 9, 7, 6, 6, 9, 1, 8],
            [0, 0, 5, 3, 9, 7, 6, 9, 3, 3],
            [0, 0, 0, 4, 2, 9, 7, 8, 2, 2],
            [0, 0, 0, 4, 2, 2, 9, 8, 9, 2],
            [0, 0, 5, 3, 2, 2, 2, 8, 7, 7],
            [0, 5, 3, 2, 2, 2, 2, 9, 6, 6],
            [9, 3, 2, 2, 2, 2, 8, 9, 6, 6],
            [7, 9, 2, 2, 2, 8, 6, 8, 6, 6],
            [6, 7, 8, 9, 9, 9, 8, 7, 6, 6],
        ]);

        let flashes = subject.steps(100);
        assert_eq!(flashes, 1656);
        assert_eq!(subject, step_10);
    }

    #[test]
    fn solve_part_b() {
        let subject = OctopusPod::from([
            [5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
            [2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
            [5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
            [6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
            [6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
            [4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
            [2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
            [6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
            [4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
            [5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
        ]);

        let result = solve_b(subject);
        assert_eq!(result, 195);
    }
}