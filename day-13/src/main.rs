use std::{collections::HashSet, fmt::Display, str::FromStr};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("input.txt");
    let (paper, folds) = Paper::parse(input)?;
    let after_first_fold = paper.fold(folds[0]);
    println!("{}", after_first_fold.marks.len());

    Ok(())
}

struct Paper {
    height: usize,
    width: usize,
    marks: HashSet<Coordinate>,
}

impl Paper {
    pub fn mark(&mut self, coordinate: Coordinate) -> bool {
        self.marks.insert(coordinate)
    }

    pub fn is_marked(&self, coordinate: Coordinate) -> bool {
        self.marks.contains(&coordinate)
    }

    pub fn fold(&self, fold: Fold) -> Self {
        match fold {
            Fold::Horizontal(h) => self.fold_horizontal(h),
            Fold::Vertical(v) => self.fold_vertical(v),
        }
    }

    fn fold_vertical(&self, fold_y: usize) -> Self {
        let marks = HashSet::new();
        let height = fold_y;
        let width = self.width;

        let mut paper = Paper {
            marks,
            width,
            height,
        };

        for y in 0..height {
            for x in 0..width {
                let coordinate = Coordinate { x, y };
                if self.is_marked(coordinate) {
                    paper.mark(coordinate);
                }
            }
        }

        for y in (fold_y + 1)..self.height {
            for x in 0..width {
                let new_y = fold_y - (y - fold_y);
                let original_coordinate = Coordinate { x, y };
                let new_coordinate = Coordinate { x, y: new_y };
                if self.is_marked(original_coordinate) {
                    paper.mark(new_coordinate);
                }
            }
        }

        paper
    }

    fn fold_horizontal(&self, fold_x: usize) -> Self {
        let marks = HashSet::new();
        let height = self.height;
        let width = fold_x;

        let mut paper = Paper {
            marks,
            width,
            height,
        };

        for y in 0..height {
            for x in 0..width {
                let coordinate = Coordinate { x, y };
                if self.is_marked(coordinate) {
                    paper.mark(coordinate);
                }
            }
        }

        for y in 0..height {
            for x in (fold_x + 1)..self.width {
                let new_x = fold_x - (x - fold_x);
                let original_coordinate = Coordinate { x, y };
                let new_coordinate = Coordinate { x: new_x, y };
                if self.is_marked(original_coordinate) {
                    paper.mark(new_coordinate);
                }
            }
        }

        paper
    }

    pub fn parse(s: &str) -> Result<(Paper, Vec<Fold>), &'static str> {
        let mut marks = HashSet::new();
        let mut max_x = 0;
        let mut max_y = 0;

        let mut folds = Vec::new();

        let mut is_marks = true;

        for line in s.lines() {
            if is_marks {
                if line.trim().is_empty() {
                    is_marks = false;
                    continue;
                }

                let coordinate: Coordinate = line.parse()?;
                max_x = std::cmp::max(max_x, coordinate.x + 1);
                max_y = std::cmp::max(max_y, coordinate.y + 1);
                marks.insert(coordinate);
            } else {
                let fold = line.parse()?;
                folds.push(fold);
            }
        }

        let paper = Paper {
            marks,
            height: max_y,
            width: max_x,
        };

        Ok((paper, folds))
    }
}

impl Display for Paper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let coordinate = Coordinate { x, y };
                if self.is_marked(coordinate) {
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

#[derive(Copy, Clone, Debug)]
pub enum Fold {
    Vertical(usize),
    Horizontal(usize),
}

impl FromStr for Fold {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rest = s
            .strip_prefix("fold along ")
            .ok_or("Expected 'fold along '")?;

        if let Some(y) = rest.strip_prefix("y=") {
            let y = y.parse().map_err(|_| "Failed to parse y=")?;
            Ok(Fold::Vertical(y))
        } else if let Some(x) = rest.strip_prefix("x=") {
            let x = x.parse().map_err(|_| "Failed to parse x=")?;
            Ok(Fold::Horizontal(x))
        } else {
            Err("Expected 'y=' or 'x='")
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl FromStr for Coordinate {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(",").ok_or("Expected comma")?;
        let x = x.parse().map_err(|_| "Invalid x")?;
        let y = y.parse().map_err(|_| "Invalid x")?;
        Ok(Coordinate { x, y })
    }
}
