const INPUT: &[usize] = &[
    3, 5, 2, 5, 4, 3, 2, 2, 3, 5, 2, 3, 2, 2, 2, 2, 3, 5, 3, 5, 5, 2, 2, 3, 4, 2, 3, 5, 5, 3, 3, 5,
    2, 4, 5, 4, 3, 5, 3, 2, 5, 4, 1, 1, 1, 5, 1, 4, 1, 4, 3, 5, 2, 3, 2, 2, 2, 5, 2, 1, 2, 2, 2, 2,
    3, 4, 5, 2, 5, 4, 1, 3, 1, 5, 5, 5, 3, 5, 3, 1, 5, 4, 2, 5, 3, 3, 5, 5, 5, 3, 2, 2, 1, 1, 3, 2,
    1, 2, 2, 4, 3, 4, 1, 3, 4, 1, 2, 2, 4, 1, 3, 1, 4, 3, 3, 1, 2, 3, 1, 3, 4, 1, 1, 2, 5, 1, 2, 1,
    2, 4, 1, 3, 2, 1, 1, 2, 4, 3, 5, 1, 3, 2, 1, 3, 2, 3, 4, 5, 5, 4, 1, 3, 4, 1, 2, 3, 5, 2, 3, 5,
    2, 1, 1, 5, 5, 4, 4, 4, 5, 3, 3, 2, 5, 4, 4, 1, 5, 1, 5, 5, 5, 2, 2, 1, 2, 4, 5, 1, 2, 1, 4, 5,
    4, 2, 4, 3, 2, 5, 2, 2, 1, 4, 3, 5, 4, 2, 1, 1, 5, 1, 4, 5, 1, 2, 5, 5, 1, 4, 1, 1, 4, 5, 2, 5,
    3, 1, 4, 5, 2, 1, 3, 1, 3, 3, 5, 5, 1, 4, 1, 3, 2, 2, 3, 5, 4, 3, 2, 5, 1, 1, 1, 2, 2, 5, 3, 4,
    2, 1, 3, 2, 5, 3, 2, 2, 3, 5, 2, 1, 4, 5, 4, 4, 5, 5, 3, 3, 5, 4, 5, 5, 4, 3, 5, 3, 5, 3, 1, 3,
    2, 2, 1, 4, 4, 5, 2, 2, 4, 2, 1, 4,
];
const INITIAL: LaternfishSchool = LaternfishSchool::from_fish(INPUT);
const AFTER_80_GENERATIONS: LaternfishSchool = INITIAL.generations(80);
const SOLUTION_A: usize = AFTER_80_GENERATIONS.count();
const AFTER_256_GENERATIONS: LaternfishSchool = INITIAL.generations(256);
const SOLUTION_B: usize = AFTER_256_GENERATIONS.count();

fn main() {
    println!("{}", SOLUTION_A);
    println!("{}", SOLUTION_B);
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct LaternfishSchool {
    number_at_age: [usize; 9],
}

impl LaternfishSchool {
    pub const fn from_fish(fish: &[usize]) -> Self {
        let mut number_at_age = [0; 9];
        let fish_len = fish.len();
        let mut i = 0;
        while i < fish_len {
            number_at_age[fish[i]] += 1;

            i += 1;
        }

        Self { number_at_age }
    }

    pub const fn generation(self) -> Self {
        let mut new = [0; 9];
        let old = self.number_at_age;

        new[0] = old[1];
        new[1] = old[2];
        new[2] = old[3];
        new[3] = old[4];
        new[4] = old[5];
        new[5] = old[6];
        new[6] = old[7] + old[0];
        new[7] = old[8];
        new[8] = old[0];

        Self { number_at_age: new }
    }

    pub const fn generations(self, mut gens: usize) -> Self {
        let mut r = self;
        while gens > 0 {
            r = r.generation();
            gens -= 1;
        }
        r
    }

    pub const fn count(&self) -> usize {
        let a = self.number_at_age;
        a[0] + a[1] + a[2] + a[3] + a[4] + a[5] + a[6] + a[7] + a[8]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let gen_0 = LaternfishSchool::from_fish(&[3, 4, 3, 1, 2]);
        let gen_1 = LaternfishSchool::from_fish(&[2, 3, 2, 0, 1]);
        let gen_2 = LaternfishSchool::from_fish(&[1, 2, 1, 6, 0, 8]);
        let gen_18 = LaternfishSchool::from_fish(&[
            6, 0, 6, 4, 5, 6, 0, 1, 1, 2, 6, 0, 1, 1, 1, 2, 2, 3, 3, 4, 6, 7, 8, 8, 8, 8,
        ]);

        assert_eq!(gen_0.generation(), gen_1);
        assert_eq!(gen_0.generations(2), gen_2);
        assert_eq!(gen_0.generations(18), gen_18);
        assert_eq!(gen_0.generations(80).count(), 5934);
        assert_eq!(gen_0.generations(256).count(), 26984457539);
    }
}
