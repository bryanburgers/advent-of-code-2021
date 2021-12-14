use std::{
    collections::{BTreeMap, HashMap},
    ops::AddAssign,
    str::FromStr,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Input {
        polymer,
        replacement_rules,
    } = include_str!("input.txt").parse()?;

    let result = solve(&polymer, &replacement_rules, 10);
    println!("{}", result);
    let result = solve(&polymer, &replacement_rules, 40);
    println!("{}", result);

    Ok(())
}

fn solve(
    polymer: &Polymer,
    replacement_rules: &BTreeMap<(char, char), char>,
    iters: usize,
) -> usize {
    let mut counter_builder = MemoizingCounterBuilder::new(replacement_rules);
    let mut counter = Counter::default();
    for window in polymer.0.windows(2) {
        let l = window[0];
        let r = window[1];
        counter += counter_builder.build_counter(l, r, iters);
    }
    counter.inc(polymer.0[polymer.0.len() - 1]);

    let incidence_map = counter.0;
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

#[derive(Clone, Default)]
struct Counter(HashMap<char, usize>);

impl Counter {
    fn inc(&mut self, ch: char) {
        self.0.entry(ch).and_modify(|v| *v += 1).or_insert(1);
    }
}

impl AddAssign for Counter {
    fn add_assign(&mut self, rhs: Self) {
        for (ch, val) in rhs.0 {
            self.0.entry(ch).and_modify(|v| *v += val).or_insert(val);
        }
    }
}

struct MemoizingCounterBuilder<'a> {
    replacement_rules: &'a BTreeMap<(char, char), char>,
    memoizer: HashMap<(char, char, usize), Counter>,
}

impl<'a> MemoizingCounterBuilder<'a> {
    fn new(replacement_rules: &'a BTreeMap<(char, char), char>) -> Self {
        Self {
            replacement_rules,
            memoizer: HashMap::new(),
        }
    }

    /// Build the counts for X-
    fn build_counter(&mut self, l: char, r: char, depth: usize) -> Counter {
        if depth == 0 {
            let mut c = Counter::default();
            c.inc(l);
            return c;
        }
        if let Some(counter) = self.memoizer.get(&(l, r, depth)) {
            return counter.clone();
        }

        let m = *self
            .replacement_rules
            .get(&(l, r))
            .expect("Expected all possible pairs to have a replacement");

        let mut l_counter = self.build_counter(l, m, depth - 1);
        let r_counter = self.build_counter(m, r, depth - 1);
        l_counter += r_counter;
        self.memoizer.insert((l, r, depth), l_counter.clone());
        l_counter
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve_a() {
        let input = include_str!("example.txt");
        let Input {
            polymer,
            replacement_rules,
        } = input.parse().unwrap();

        let result = solve(&polymer, &replacement_rules, 10);
        assert_eq!(result, 1588);
    }
}
