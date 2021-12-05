use std::{collections::HashMap, str::FromStr};

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    let segments = parse(input)?;
    let solution_a = solve_a(&segments);
    println!("{}", solution_a);
    let solution_b = solve_b(&segments);
    println!("{}", solution_b);
    Ok(())
}

fn solve_a(segments: &[LineSegment]) -> usize {
    let mut points_seen = HashMap::new();
    for segment in segments {
        if segment.is_horizontal() || segment.is_vertical() {
            for point in segment.points() {
                points_seen
                    .entry(point)
                    .and_modify(|v| *v += 1)
                    .or_insert(1_usize);
            }
        }
    }

    points_seen
        .into_iter()
        .filter(|(_point, seen)| *seen > 1)
        .count()
}

fn solve_b(segments: &[LineSegment]) -> usize {
    let mut points_seen = HashMap::new();
    for segment in segments {
        for point in segment.points() {
            points_seen
                .entry(point)
                .and_modify(|v| *v += 1)
                .or_insert(1_usize);
        }
    }

    points_seen
        .into_iter()
        .filter(|(_point, seen)| *seen > 1)
        .count()
}

fn parse(s: &str) -> Result<Vec<LineSegment>> {
    s.lines()
        .map(|s| s.parse::<LineSegment>().map_err(Into::into))
        .collect::<Result<Vec<_>>>()
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').ok_or("expected comma")?;
        let x = x.parse().map_err(|_| "failed to parse x")?;
        let y = y.parse().map_err(|_| "failed to parse y")?;
        Ok(Self { x, y })
    }
}

#[derive(Debug)]
struct LineSegment {
    start: Point,
    end: Point,
}

impl FromStr for LineSegment {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once(" -> ").ok_or("expected arrow")?;
        let start = start.parse().map_err(|_| "failed to parse start")?;
        let end = end.parse().map_err(|_| "failed to parse end")?;
        Ok(Self { start, end })
    }
}

impl LineSegment {
    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    fn points(&self) -> PointsIterator {
        PointsIterator {
            start: self.start,
            end: self.end,
            done: false,
        }
    }
}

struct PointsIterator {
    start: Point,
    end: Point,
    done: bool,
}

impl Iterator for PointsIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        if self.start == self.end {
            self.done = true;
            return Some(self.start);
        }

        if self.start.x == self.end.x
            || self.start.y == self.end.y
            || (self.start.x - self.end.x).abs() == (self.start.y - self.end.y).abs()
        {
        } else {
            panic!(
                "Not horizontal, vertical, or perfectly diagonal. {:?} {:?}",
                self.start, self.end
            );
        }

        let ret = self.start;

        if self.start.y > self.end.y {
            self.start.y -= 1;
        } else if self.start.y < self.end.y {
            self.start.y += 1;
        }

        if self.start.x > self.end.x {
            self.start.x -= 1;
        } else if self.start.x < self.end.x {
            self.start.x += 1;
        }

        return Some(ret);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_horizontal() {
        let subject = LineSegment {
            start: Point { x: 0, y: 9 },
            end: Point { x: 5, y: 9 },
        };

        assert!(subject.is_horizontal());

        let subject = LineSegment {
            start: Point { x: 0, y: 8 },
            end: Point { x: 8, y: 0 },
        };

        assert!(!subject.is_horizontal());
    }

    #[test]
    fn is_vertical() {
        let subject = LineSegment {
            start: Point { x: 9, y: 0 },
            end: Point { x: 9, y: 5 },
        };

        assert!(subject.is_vertical());

        let subject = LineSegment {
            start: Point { x: 0, y: 8 },
            end: Point { x: 8, y: 0 },
        };

        assert!(!subject.is_vertical());
    }

    #[test]
    fn points() {
        let subject = LineSegment {
            start: Point { x: 0, y: 9 },
            end: Point { x: 5, y: 9 },
        };

        let points = subject.points().collect::<Vec<_>>();
        assert_eq!(
            points,
            vec![
                Point { x: 0, y: 9 },
                Point { x: 1, y: 9 },
                Point { x: 2, y: 9 },
                Point { x: 3, y: 9 },
                Point { x: 4, y: 9 },
                Point { x: 5, y: 9 },
            ]
        );

        let subject = LineSegment {
            start: Point { x: 9, y: 0 },
            end: Point { x: 9, y: 5 },
        };

        let points = subject.points().collect::<Vec<_>>();
        assert_eq!(
            points,
            vec![
                Point { x: 9, y: 0 },
                Point { x: 9, y: 1 },
                Point { x: 9, y: 2 },
                Point { x: 9, y: 3 },
                Point { x: 9, y: 4 },
                Point { x: 9, y: 5 },
            ]
        );
    }

    #[test]
    fn points_diagonal() {
        let subject = LineSegment {
            start: Point { x: 1, y: 1 },
            end: Point { x: 3, y: 3 },
        };

        let points = subject.points().collect::<Vec<_>>();
        assert_eq!(
            points,
            vec![
                Point { x: 1, y: 1 },
                Point { x: 2, y: 2 },
                Point { x: 3, y: 3 },
            ]
        );

        let subject = LineSegment {
            start: Point { x: 9, y: 7 },
            end: Point { x: 7, y: 9 },
        };

        let points = subject.points().collect::<Vec<_>>();
        assert_eq!(
            points,
            vec![
                Point { x: 9, y: 7 },
                Point { x: 8, y: 8 },
                Point { x: 7, y: 9 },
            ]
        );
    }

    #[test]
    fn test_a() {
        let input = include_str!("example.txt");
        let input = parse(input).unwrap();
        assert_eq!(solve_a(&input), 5);
    }

    #[test]
    fn test_b() {
        let input = include_str!("example.txt");
        let input = parse(input).unwrap();
        assert_eq!(solve_b(&input), 12);
    }
}
