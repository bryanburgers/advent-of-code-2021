use std::{collections::BTreeMap, str::FromStr};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Input {
        polymer,
        replacement_rules,
    } = include_str!("input.txt").parse()?;

    let result = solve_a(&polymer, &replacement_rules);
    println!("{}", result);

    Ok(())
}

fn solve_a(polymer: &Polymer, replacement_rules: &BTreeMap<(char, char), char>) -> usize {
    let mut polymer = polymer.clone();
    for _ in 0..10 {
        polymer = polymer.step(replacement_rules);
    }

    let mut incidence_map = BTreeMap::new();
    for ch in polymer.0 {
        incidence_map.entry(ch).and_modify(|v| *v += 1).or_insert(1);
    }
    let mut incidence_vec = incidence_map.into_iter().collect::<Vec<_>>();
    incidence_vec.sort_by(|&(_, a), &(_, b)| a.cmp(&b).reverse());
    let most = incidence_vec[0].1;
    let least = incidence_vec.last().unwrap().1;

    most - least
}

struct Input {
    polymer: Polymer,
    replacement_rules: BTreeMap<(char, char), char>,
}

impl FromStr for Input {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let polymer_str = lines.next().ok_or("Missing polymer")?;
        let polymer = polymer_str.parse()?;
        let _blank = lines.next().ok_or("Missing blank line")?;

        let mut replacement_rules = BTreeMap::default();
        for line in lines {
            let (left, right) = line.split_once(" -> ").ok_or("Missing separator")?;
            if left.len() != 2 {
                return Err("Left was not 2 characters");
            }
            if right.len() != 1 {
                return Err("Right was not 1 characters");
            }
            let mut left_chars = left.chars();
            let l1 = left_chars.next().unwrap();
            let l2 = left_chars.next().unwrap();
            let mut right_chars = right.chars();
            let r = right_chars.next().unwrap();

            replacement_rules.insert((l1, l2), r);
        }

        Ok(Input {
            polymer,
            replacement_rules,
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Polymer(Vec<char>);

impl FromStr for Polymer {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.chars().collect()))
    }
}

impl Polymer {
    fn step(&self, replacement_rules: &BTreeMap<(char, char), char>) -> Self {
        let mut new_vec = Vec::with_capacity(self.0.len() * 2 - 1);
        for window in self.0.windows(2) {
            let ch1 = window[0];
            let ch2 = window[1];
            new_vec.push(ch1);
            if let Some(r) = replacement_rules.get(&(ch1, ch2)) {
                new_vec.push(*r);
            }
        }
        new_vec.push(self.0[self.0.len() - 1]);
        Self(new_vec)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_step() {
        let input = include_str!("example.txt");
        let Input {
            polymer,
            replacement_rules,
        } = input.parse().unwrap();

        let after_step_1 = polymer.step(&replacement_rules);
        assert_eq!(after_step_1, "NCNBCHB".parse().unwrap());

        let after_step_2 = after_step_1.step(&replacement_rules);
        assert_eq!(after_step_2, "NBCCNBBBCBHCB".parse().unwrap());

        let after_step_3 = after_step_2.step(&replacement_rules);
        assert_eq!(after_step_3, "NBBBCNCCNBBNBNBBCHBHHBCHB".parse().unwrap());

        let after_step_4 = after_step_3.step(&replacement_rules);
        assert_eq!(
            after_step_4,
            "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB"
                .parse()
                .unwrap()
        );
    }

    #[test]
    fn test_solve_a() {
        let input = include_str!("example.txt");
        let Input {
            polymer,
            replacement_rules,
        } = input.parse().unwrap();

        let result = solve_a(&polymer, &replacement_rules);
        assert_eq!(result, 1588);
    }
}
