type Result<T, E = Box<dyn std::error::Error + Send + Sync>> = std::result::Result<T, E>;

fn main() -> Result<()> {
    let str = include_str!("input.txt");
    let parsed = parse(str)?;
    let mut evaluator = DiagnosticReporter::<12>::default();
    for input in &parsed {
        evaluator.evaluate(*input);
    }
    let gamma = evaluator.gamma_rate();
    let epsilon = evaluator.epsilon_rate();
    println!(
        "{} • gamma={} • epsilon={}",
        gamma as u64 * epsilon as u64,
        gamma,
        epsilon
    );

    let oxygen = RatingFilter::find_rating(RatingFilterStrategy::Oxygen, parsed.clone(), 12);
    let co2 = RatingFilter::find_rating(RatingFilterStrategy::Co2, parsed, 12);
    println!(
        "{} • oxygen={} • co2={}",
        oxygen as u64 * co2 as u64,
        oxygen,
        co2
    );

    Ok(())
}

fn parse(input: &str) -> Result<Vec<u16>> {
    input
        .lines()
        .map(|line| u16::from_str_radix(line, 2).map_err(Into::into))
        .collect::<Result<_>>()
}

#[derive(Debug)]
pub struct DiagnosticReporter<const BITS: usize> {
    seen: usize,
    counts: [usize; BITS],
}

impl<const BITS: usize> Default for DiagnosticReporter<BITS> {
    fn default() -> Self {
        let seen = 0;
        let counts = [0; BITS];
        Self { seen, counts }
    }
}

impl<const BITS: usize> DiagnosticReporter<BITS> {
    fn mask() -> u16 {
        assert!(BITS < u16::BITS as usize);

        (1 << BITS) - 1
    }

    pub fn evaluate(&mut self, input: u16) {
        for idx in 0..BITS {
            let mask = 1_u16 << idx;
            let masked = input & mask;
            let shifted = masked >> idx;
            self.counts[idx] += shifted as usize;
        }
        self.seen += 1;
    }

    pub fn gamma_rate(&self) -> u16 {
        let half = self.seen / 2;
        let mut result = 0;

        for idx in 0..BITS {
            let v = if self.counts[idx] > half { 1 } else { 0 };
            result |= v << idx;
        }

        result
    }

    pub fn epsilon_rate(&self) -> u16 {
        let gamma = self.gamma_rate();
        let epsilon_unmasked = !gamma;
        epsilon_unmasked & Self::mask()
    }
}

#[derive(Copy, Clone, Debug)]
enum RatingFilterStrategy {
    Oxygen,
    Co2,
}

impl RatingFilterStrategy {
    pub fn keep(self, ones: usize, zeros: usize) -> u16 {
        match (self, ones, zeros) {
            (Self::Oxygen, ones, zeros) if ones < zeros => 0,
            (Self::Oxygen, _, _) => 1,
            (Self::Co2, ones, zeros) if ones < zeros => 1,
            (Self::Co2, _, _) => 0,
        }
    }
}

struct RatingFilter {
    strategy: RatingFilterStrategy,
}

impl RatingFilter {
    pub fn new(strategy: RatingFilterStrategy) -> Self {
        Self { strategy }
    }

    pub fn filter(&self, idx: usize, items: Vec<u16>) -> Vec<u16> {
        let len = items.len();
        if len <= 1 {
            return items;
        }

        let mut ones = 0;

        for item in &items {
            let item = *item;
            let is_one = (item >> idx) & 0b1;
            ones += is_one as usize;
        }

        let keep = self.strategy.keep(ones, len - ones);

        let is_match = |v: &u16| -> bool { v >> idx & 0b1 == keep };

        items.into_iter().filter(is_match).collect()
    }

    pub fn find_rating(strategy: RatingFilterStrategy, mut items: Vec<u16>, bits: usize) -> u16 {
        let mut idx = bits - 1;
        assert!(!items.is_empty());
        let filter = RatingFilter::new(strategy);

        while items.len() > 1 {
            items = filter.filter(idx, items);

            if items.len() <= 1 {
                break;
            }

            if idx == 0 {
                panic!("Not found");
            }
            idx -= 1;
        }

        assert!(!items.is_empty());
        items[0]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mask() {
        assert_eq!(DiagnosticReporter::<1>::mask(), 0b1);
        assert_eq!(DiagnosticReporter::<2>::mask(), 0b11);
        assert_eq!(DiagnosticReporter::<3>::mask(), 0b111);
        assert_eq!(DiagnosticReporter::<5>::mask(), 0b11111);
        assert_eq!(DiagnosticReporter::<12>::mask(), 0b111111111111);
    }

    #[test]
    fn test_1() -> Result<()> {
        let str = include_str!("example.txt");
        let parsed = parse(str)?;
        let mut evaluator = DiagnosticReporter::<5>::default();
        for input in &parsed {
            evaluator.evaluate(*input);
        }

        assert_eq!(evaluator.gamma_rate(), 22);
        assert_eq!(evaluator.epsilon_rate(), 9);

        Ok(())
    }

    #[test]
    fn strategy() {
        let oxygen = RatingFilterStrategy::Oxygen;

        assert_eq!(
            oxygen.keep(2, 1),
            1,
            "If there are more 1s than 0s, keep 1s"
        );
        assert_eq!(
            oxygen.keep(1, 2),
            0,
            "If there are more 0s than 1s, keep 0s"
        );
        assert_eq!(
            oxygen.keep(1, 1),
            1,
            "If there are the same amount of 1s and 0s, keep 1s"
        );

        let co2 = RatingFilterStrategy::Co2;

        assert_eq!(co2.keep(2, 1), 0, "If there are more 1s than 0s, keep 0s");
        assert_eq!(co2.keep(1, 2), 1, "If there are more 0s than 1s, keep 1s");
        assert_eq!(
            co2.keep(1, 1),
            0,
            "If there are the same amount of 1s and 0s, keep 0s"
        );
    }

    #[test]
    fn test_2() -> Result<()> {
        let str = include_str!("example.txt");
        let parsed = parse(str)?;
        let oxygen = RatingFilter::find_rating(RatingFilterStrategy::Oxygen, parsed.clone(), 5);
        assert_eq!(oxygen, 0b10111);

        let co2 = RatingFilter::find_rating(RatingFilterStrategy::Co2, parsed.clone(), 5);
        assert_eq!(co2, 0b01010);

        Ok(())
    }
}
