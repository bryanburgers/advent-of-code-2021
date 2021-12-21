fn main() {
    let result = solve_a(10, 8);
    println!("{}", result);
}

fn solve_a(player_1_start: usize, player_2_start: usize) -> usize {
    let player_1 = Player::new(Pawn::new(player_1_start));
    let player_2 = Player::new(Pawn::new(player_2_start));
    let mut die = DeterministicDie::new();

    let mut players = [player_1, player_2];
    let mut current_player_idx = 0;

    loop {
        let current_player = &mut players[current_player_idx];
        current_player.take_turn(&mut die);

        if current_player.score >= 1000 {
            let other_player_idx = (current_player_idx + 1) % 2;
            let other_player = &players[other_player_idx];
            return other_player.score * die.rolls();
        }

        current_player_idx = (current_player_idx + 1) % 2;
    }
}

struct Player {
    pawn: Pawn,
    score: usize,
}

impl Player {
    pub fn new(pawn: Pawn) -> Self {
        Self { pawn, score: 0 }
    }

    pub fn take_turn(&mut self, die: &mut DeterministicDie) {
        let roll_1 = die.next().unwrap();
        let roll_2 = die.next().unwrap();
        let roll_3 = die.next().unwrap();
        self.pawn.move_pawn(roll_1 + roll_2 + roll_3);
        let current_space = self.pawn.current_space;
        self.score += current_space;
    }
}

struct Pawn {
    current_space: usize,
}

impl Pawn {
    pub fn new(current_space: usize) -> Self {
        if current_space < 1 || current_space > 10 {
            panic!("Nope!");
        }

        Self { current_space }
    }
    pub fn move_pawn(&mut self, value: usize) {
        let new_space = self.current_space + value;
        let mut normalized_new_space = new_space % 10;
        if normalized_new_space == 0 {
            normalized_new_space = 10;
        }

        self.current_space = normalized_new_space;
    }
}

struct DeterministicDie {
    rolls: usize,
}

impl DeterministicDie {
    pub fn new() -> DeterministicDie {
        Self { rolls: 0 }
    }

    pub fn rolls(&self) -> usize {
        self.rolls
    }
}

impl Iterator for DeterministicDie {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.rolls += 1;
        let mut value = self.rolls % 100;
        if value == 0 {
            value = 100;
        }

        Some(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deterministic_die() {
        let mut die = DeterministicDie::new();
        assert_eq!(die.next(), Some(1));
        assert_eq!(die.next(), Some(2));
        assert_eq!(die.next(), Some(3));
        assert_eq!(die.rolls(), 3);

        for _ in 4..=98 {
            die.next();
        }

        assert_eq!(die.next(), Some(99));
        assert_eq!(die.next(), Some(100));
        assert_eq!(die.next(), Some(1));
        assert_eq!(die.rolls(), 101);
    }

    #[test]
    fn test_pawn() {
        let mut pawn = Pawn::new(4);
        pawn.move_pawn(1);
        pawn.move_pawn(2);
        pawn.move_pawn(3);
        assert_eq!(pawn.current_space, 10);
        pawn.move_pawn(1);
        assert_eq!(pawn.current_space, 1);

        let mut pawn = Pawn::new(8);
        pawn.move_pawn(4);
        pawn.move_pawn(5);
        pawn.move_pawn(6);
        assert_eq!(pawn.current_space, 3);
    }

    #[test]
    fn test_players() {
        let mut player_1 = Player::new(Pawn::new(4));
        let mut player_2 = Player::new(Pawn::new(8));
        let mut die = DeterministicDie::new();

        player_1.take_turn(&mut die);
        assert_eq!(player_1.score, 10);
        player_2.take_turn(&mut die);
        assert_eq!(player_2.score, 3);
        player_1.take_turn(&mut die);
        assert_eq!(player_1.score, 14);
        player_2.take_turn(&mut die);
        assert_eq!(player_2.score, 9);
    }

    #[test]
    fn test_solve_a() {
        let result = solve_a(4, 8);
        assert_eq!(result, 739785);
    }
}
