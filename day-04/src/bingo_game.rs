use crate::bingo_board::BingoBoard;
use crate::coordinate::Coordinate;

#[derive(Copy, Clone)]
pub struct BingoGame<'a> {
    board: &'a BingoBoard,
    chips: Chips,
}

impl<'a> BingoGame<'a> {
    pub const fn new(board: &BingoBoard) -> BingoGame {
        BingoGame {
            board,
            chips: Chips::new(),
        }
    }

    pub fn call(&mut self, val: u8) {
        if let Some(coordinate) = self.board.find(val) {
            self.chips.set_called(coordinate);
        }
    }

    pub const fn with_called(mut self, val: u8) -> Self {
        if let Some(coordinate) = self.board.find(val) {
            self.chips = self.chips.called(coordinate);
        }
        self
    }

    pub const fn has_won(&self) -> bool {
        let mut i = 0;
        let len = WINNING_POSITIONS.len();
        while i < len {
            if self.chips.contains(&WINNING_POSITIONS[i]) {
                return true;
            }
            i += 1;
        }
        false
    }

    pub const fn unmarked_sum(&self) -> u16 {
        let mut sum = 0;
        let mut i = 0;
        while i < 25 {
            let coordinate = Coordinate::from_index(i);
            if !self.chips.is_called(coordinate) {
                sum += self.board.at(coordinate) as u16;
            }
            i += 1;
        }

        sum
    }

    pub const fn play(self, calls: &[u8]) -> Endgame {
        let mut this = self;
        let mut i = 0;
        let len = calls.len();
        while i < len {
            let call = calls[i];
            this = this.with_called(call);
            if this.has_won() {
                return Endgame {
                    turns: i + 1,
                    last_call: call,
                    unmarked_sum: this.unmarked_sum(),
                };
            }
            i += 1;
        }

        panic!("Did not finish");
    }
}

pub struct Endgame {
    pub turns: usize,
    pub last_call: u8,
    pub unmarked_sum: u16,
}

#[derive(Copy, Clone, Default)]
/// In a physical game of bingo, chips on a board are used to represent which numbers
/// have been called.
///
/// This `Chips` struct does the same thing: it tracks which numbers on the board have
/// been called, by tracking the positions of those chips.
struct Chips(u32);

impl Chips {
    pub const fn new() -> Self {
        Self(0)
    }

    pub const fn called(mut self, coordinate: Coordinate) -> Self {
        self.0 |= 1 << coordinate.index();
        self
    }

    pub fn set_called(&mut self, coordinate: Coordinate) {
        self.0 |= 1 << coordinate.index();
    }

    pub const fn is_called(&self, coordinate: Coordinate) -> bool {
        (self.0 >> coordinate.index() & 0b1) == 1
    }

    pub const fn contains(&self, other: &Chips) -> bool {
        self.0 & other.0 == other.0
    }
}

const WINNING_POSITIONS: &[Chips] = &[
    WINNING_POSITION_H0,
    WINNING_POSITION_H1,
    WINNING_POSITION_H2,
    WINNING_POSITION_H3,
    WINNING_POSITION_H4,
    WINNING_POSITION_V0,
    WINNING_POSITION_V1,
    WINNING_POSITION_V2,
    WINNING_POSITION_V3,
    WINNING_POSITION_V4,
];

const WINNING_POSITION_H0: Chips = Chips::new()
    .called(Coordinate::from_pos(0, 0))
    .called(Coordinate::from_pos(1, 0))
    .called(Coordinate::from_pos(2, 0))
    .called(Coordinate::from_pos(3, 0))
    .called(Coordinate::from_pos(4, 0));
const WINNING_POSITION_H1: Chips = Chips::new()
    .called(Coordinate::from_pos(0, 1))
    .called(Coordinate::from_pos(1, 1))
    .called(Coordinate::from_pos(2, 1))
    .called(Coordinate::from_pos(3, 1))
    .called(Coordinate::from_pos(4, 1));
const WINNING_POSITION_H2: Chips = Chips::new()
    .called(Coordinate::from_pos(0, 2))
    .called(Coordinate::from_pos(1, 2))
    .called(Coordinate::from_pos(2, 2))
    .called(Coordinate::from_pos(3, 2))
    .called(Coordinate::from_pos(4, 2));
const WINNING_POSITION_H3: Chips = Chips::new()
    .called(Coordinate::from_pos(0, 3))
    .called(Coordinate::from_pos(1, 3))
    .called(Coordinate::from_pos(2, 3))
    .called(Coordinate::from_pos(3, 3))
    .called(Coordinate::from_pos(4, 3));
const WINNING_POSITION_H4: Chips = Chips::new()
    .called(Coordinate::from_pos(0, 4))
    .called(Coordinate::from_pos(1, 4))
    .called(Coordinate::from_pos(2, 4))
    .called(Coordinate::from_pos(3, 4))
    .called(Coordinate::from_pos(4, 4));
const WINNING_POSITION_V0: Chips = Chips::new()
    .called(Coordinate::from_pos(0, 0))
    .called(Coordinate::from_pos(0, 1))
    .called(Coordinate::from_pos(0, 2))
    .called(Coordinate::from_pos(0, 3))
    .called(Coordinate::from_pos(0, 4));
const WINNING_POSITION_V1: Chips = Chips::new()
    .called(Coordinate::from_pos(1, 0))
    .called(Coordinate::from_pos(1, 1))
    .called(Coordinate::from_pos(1, 2))
    .called(Coordinate::from_pos(1, 3))
    .called(Coordinate::from_pos(1, 4));
const WINNING_POSITION_V2: Chips = Chips::new()
    .called(Coordinate::from_pos(2, 0))
    .called(Coordinate::from_pos(2, 1))
    .called(Coordinate::from_pos(2, 2))
    .called(Coordinate::from_pos(2, 3))
    .called(Coordinate::from_pos(2, 4));
const WINNING_POSITION_V3: Chips = Chips::new()
    .called(Coordinate::from_pos(3, 0))
    .called(Coordinate::from_pos(3, 1))
    .called(Coordinate::from_pos(3, 2))
    .called(Coordinate::from_pos(3, 3))
    .called(Coordinate::from_pos(3, 4));
const WINNING_POSITION_V4: Chips = Chips::new()
    .called(Coordinate::from_pos(4, 0))
    .called(Coordinate::from_pos(4, 1))
    .called(Coordinate::from_pos(4, 2))
    .called(Coordinate::from_pos(4, 3))
    .called(Coordinate::from_pos(4, 4));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chips() {
        let mut mask = Chips::new();
        mask.set_called(Coordinate::from_pos(2, 2));
        mask.set_called(Coordinate::from_pos(1, 3));

        assert!(mask.is_called(Coordinate::from_pos(2, 2)));
        assert!(mask.is_called(Coordinate::from_pos(1, 3)));
        assert!(!mask.is_called(Coordinate::from_pos(0, 0)));
    }

    #[test]
    fn chips_contains() {
        let called = vec![
            (0, 0),
            (0, 1),
            (0, 2),
            (0, 3),
            (0, 4),
            (1, 0),
            (1, 1),
            (1, 2),
        ];

        let mut chips = Chips::new();
        for (x, y) in called {
            chips.set_called(Coordinate::from_pos(x, y));
        }

        assert!(chips.contains(&WINNING_POSITION_V0));
        assert!(!chips.contains(&WINNING_POSITION_V1));
    }

    #[test]
    fn example() {
        let board = BingoBoard::from_arrays([
            [14, 21, 17, 24, 4],
            [10, 16, 15, 9, 19],
            [18, 8, 23, 26, 20],
            [22, 11, 13, 6, 5],
            [2, 0, 12, 3, 7],
        ]);
        let calls = vec![7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24];
        let mut game = BingoGame::new(&board);
        for call in &calls {
            game = game.with_called(*call);
        }
        assert!(game.has_won());

        let mut game = BingoGame::new(&board);
        let endgame = game.play(&calls);
        assert_eq!(endgame.turns, 12);
        assert_eq!(endgame.last_call, 24);
        assert_eq!(endgame.unmarked_sum, 188);
    }
}
