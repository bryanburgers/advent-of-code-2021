use std::fmt::Debug;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Coordinate(usize);

impl Coordinate {
    pub const fn from_index(idx: usize) -> Coordinate {
        if idx >= 25 {
            panic!("Out of bounds");
        } else {
            Coordinate(idx)
        }
    }

    pub const fn from_tuple((x, y): (u8, u8)) -> Coordinate {
        if x >= 5 || y >= 5 {
            panic!("Out of bounds");
        } else {
            Self(x as usize + y as usize * 5)
        }
    }

    pub const fn from_pos(x: u8, y: u8) -> Coordinate {
        Self::from_tuple((x, y))
    }

    pub const fn index(self) -> usize {
        self.0
    }

    pub const fn tuple(self) -> (u8, u8) {
        let y = self.0 / 5;
        let x = self.0 - (y * 5);
        (x as u8, y as u8)
    }

    pub const fn x(self) -> u8 {
        self.tuple().0
    }

    pub const fn y(self) -> u8 {
        self.tuple().1
    }
}

impl Debug for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Coordinate")
            .field("x", &self.x())
            .field("y", &self.y())
            .field("index", &self.index())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_index() {
        let coordinate = Coordinate::from_index(0);
        assert_eq!(coordinate.tuple(), (0, 0));

        let coordinate = Coordinate::from_index(4);
        assert_eq!(coordinate.tuple(), (4, 0));

        let coordinate = Coordinate::from_index(5);
        assert_eq!(coordinate.tuple(), (0, 1));

        let coordinate = Coordinate::from_index(24);
        assert_eq!(coordinate.tuple(), (4, 4));

        // let result = Coordinate::from_const_index(25);
        // assert!(result.is_err());
    }

    #[test]
    fn from_tuple() {
        let coordinate = Coordinate::from_tuple((0, 0));
        assert_eq!(coordinate.tuple(), (0, 0));
        assert_eq!(coordinate.index(), 0);

        let coordinate = Coordinate::from_tuple((4, 0));
        assert_eq!(coordinate.tuple(), (4, 0));
        assert_eq!(coordinate.index(), 4);

        let coordinate = Coordinate::from_tuple((0, 1));
        assert_eq!(coordinate.tuple(), (0, 1));
        assert_eq!(coordinate.index(), 5);

        let coordinate = Coordinate::from_tuple((4, 4));
        assert_eq!(coordinate.tuple(), (4, 4));
        assert_eq!(coordinate.index(), 24);

        // let result = Coordinate::from_const_tuple((0, 5));
        // assert!(result.is_err());

        // let result = Coordinate::from_const_tuple((5, 0));
        // assert!(result.is_err());
    }
}
