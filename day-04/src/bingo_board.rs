use crate::coordinate::Coordinate;
use std::fmt::Debug;

#[derive(Copy, Clone)]
pub struct BingoBoard([u8; 25]);

impl BingoBoard {
    pub const fn from_array(arr: [u8; 25]) -> Self {
        BingoBoard(arr)
    }

    pub const fn from_arrays(arrs: [[u8; 5]; 5]) -> Self {
        let mut arr = [0; 25];
        let mut i = 0_usize;
        while i < 25 {
            let coordinate = Coordinate::from_index(i);
            arr[coordinate.index()] = arrs[coordinate.y() as usize][coordinate.x() as usize];
            i += 1;
        }
        Self(arr)
    }

    pub const fn at(&self, coordinate: Coordinate) -> u8 {
        self.0[coordinate.index()]
    }

    pub const fn find(&self, val: u8) -> Option<Coordinate> {
        let mut i = 0;
        while i < 25 {
            if self.0[i] == val {
                return Some(Coordinate::from_index(i));
            }
            i += 1;
        }
        None
    }
}

impl Debug for BingoBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:3} {:3} {:3} {:3} {:3}\n",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4]
        )?;
        write!(
            f,
            "{:3} {:3} {:3} {:3} {:3}\n",
            self.0[5], self.0[6], self.0[7], self.0[8], self.0[9]
        )?;
        write!(
            f,
            "{:3} {:3} {:3} {:3} {:3}\n",
            self.0[10], self.0[11], self.0[12], self.0[13], self.0[14]
        )?;
        write!(
            f,
            "{:3} {:3} {:3} {:3} {:3}\n",
            self.0[15], self.0[16], self.0[17], self.0[18], self.0[19]
        )?;
        write!(
            f,
            "{:3} {:3} {:3} {:3} {:3}\n",
            self.0[20], self.0[21], self.0[22], self.0[23], self.0[24]
        )?;
        Ok(())
    }
}

macro_rules! parse_input {
    { $(
        $a00:literal $a10:literal $a20:literal $a30:literal $a40:literal
        $a01:literal $a11:literal $a21:literal $a31:literal $a41:literal
        $a02:literal $a12:literal $a22:literal $a32:literal $a42:literal
        $a03:literal $a13:literal $a23:literal $a33:literal $a43:literal
        $a04:literal $a14:literal $a24:literal $a34:literal $a44:literal
    )* } => {
        &[
            $(crate::bingo_board::BingoBoard::from_arrays([
                [$a00, $a10, $a20, $a30, $a40],
                [$a01, $a11, $a21, $a31, $a41],
                [$a02, $a12, $a22, $a32, $a42],
                [$a03, $a13, $a23, $a33, $a43],
                [$a04, $a14, $a24, $a34, $a44],
            ])),*
        ]
    }
}

pub(crate) use parse_input;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_arrays() {
        let board = BingoBoard::from_arrays([
            [0, 1, 2, 3, 4],
            [5, 6, 7, 8, 9],
            [10, 11, 12, 13, 14],
            [15, 16, 17, 18, 19],
            [20, 21, 22, 23, 24],
        ]);

        for i in 0..25 {
            let coordinate = Coordinate::from_index(i);
            assert_eq!(board.at(coordinate), coordinate.index() as u8);
        }
    }

    #[test]
    fn find() {
        let board = BingoBoard::from_arrays([
            [0, 1, 2, 3, 4],
            [5, 6, 7, 8, 9],
            [10, 11, 12, 13, 14],
            [15, 16, 17, 18, 19],
            [20, 21, 22, 23, 24],
        ]);

        assert_eq!(board.find(4), Some(Coordinate::from_pos(4, 0)));
        assert_eq!(board.find(25), None);
    }
}
