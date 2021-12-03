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
}
