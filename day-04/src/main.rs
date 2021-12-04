mod bingo_board;
mod bingo_game;
mod coordinate;
mod input;

const CALLS: &[u8] = input::CALLS;
const BOARDS: &[bingo_board::BingoBoard] = input::BOARDS;
// Exceeded interpreter step limit
// const SOLUTION_A: usize = solve_a(CALLS, BOARDS);

fn main() {
    let solution_a = solve_a(CALLS, BOARDS);
    println!("{}", solution_a);
}

const fn solve_a(calls: &[u8], boards: &[bingo_board::BingoBoard]) -> usize {
    let mut endgame: Option<bingo_game::Endgame> = None;

    let len = boards.len();
    let mut i = 0;
    while i < len {
        let board = &boards[i];
        let game = bingo_game::BingoGame::new(board);
        let e = game.play(calls);
        if let Some(ref outer_endgame) = endgame {
            if e.turns < outer_endgame.turns {
                endgame = Some(e);
            }
        } else {
            endgame = Some(e)
        }

        i += 1;
    }

    if let Some(endgame) = endgame {
        endgame.last_call as usize * endgame.unmarked_sum as usize
    } else {
        panic!("None found");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        const CALLS: &[u8] = &[
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];
        const BOARDS: &[bingo_board::BingoBoard] = bingo_board::parse_input! {
            22 13 17 11  0
            8  2 23  4 24
            21  9 14 16  7
            6 10  3 18  5
            1 12 20 15 19

            3 15  0  2 22
            9 18 13 17  5
            19  8  7 25 23
            20 11 10 24  4
            14 21 16 12  6

            14 21 17 24  4
            10 16 15  9 19
            18  8 23 26 20
            22 11 13  6  5
            2  0 12  3  7
        };

        const RESULT: usize = solve_a(CALLS, BOARDS);
        assert_eq!(RESULT, 4512);
    }
}
