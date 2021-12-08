use std::{collections::HashSet, str::FromStr};

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    let input = parse(input)?;
    let count = solve_a(&input);
    println!("{}", count);

    Ok(())
}

fn parse(input: &str) -> Result<Vec<Input>> {
    input
        .lines()
        .map(|line| line.parse::<Input>().map_err(Into::into))
        .collect::<Result<Vec<_>>>()
}

fn solve_a(input: &[Input]) -> usize {
    let mut count = 0;
    for item in input {
        for signal_pattern in &item.output {
            if signal_pattern.is_counted_in_part_a() {
                count += 1;
            }
        }
    }
    count
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum SignalWire {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

#[derive(Debug, thiserror::Error)]
enum SignalWireParseError {
    #[error("Unexpected input '{0}'")]
    UnexpectedInput(String),
}

impl FromStr for SignalWire {
    type Err = SignalWireParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" => Ok(SignalWire::A),
            "b" => Ok(SignalWire::B),
            "c" => Ok(SignalWire::C),
            "d" => Ok(SignalWire::D),
            "e" => Ok(SignalWire::E),
            "f" => Ok(SignalWire::F),
            "g" => Ok(SignalWire::G),
            _ => Err(SignalWireParseError::UnexpectedInput(s.to_string())),
        }
    }
}

#[derive(Clone, Debug, Default)]
struct SignalPattern {
    signals: HashSet<SignalWire>,
}

impl SignalPattern {
    fn set(&mut self, signal_wire: SignalWire) {
        self.signals.insert(signal_wire);
    }

    fn contains(&self, signal_wire: SignalWire) -> bool {
        self.signals.contains(&signal_wire)
    }

    fn signals_set(&self) -> usize {
        self.signals.len()
    }

    fn is_one(&self) -> bool {
        self.signals_set() == 2
    }
    fn is_four(&self) -> bool {
        self.signals_set() == 4
    }
    fn is_seven(&self) -> bool {
        self.signals_set() == 3
    }
    fn is_eight(&self) -> bool {
        self.signals_set() == 7
    }
    fn is_counted_in_part_a(&self) -> bool {
        self.is_one() || self.is_four() || self.is_seven() || self.is_eight()
    }
}

#[derive(Debug, thiserror::Error)]
enum SignalPatternParseError {
    #[error("Signal wire error: {0}")]
    SignalWire(#[from] SignalWireParseError),
}

impl FromStr for SignalPattern {
    type Err = SignalPatternParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pattern = Self::default();
        for (idx, ch) in s.char_indices() {
            let signal_wire_str = &s[idx..][..ch.len_utf8()];
            let signal_wire = signal_wire_str.parse()?;
            pattern.set(signal_wire);
        }
        Ok(pattern)
    }
}

struct Input {
    unique_patterns: [SignalPattern; 10],
    output: [SignalPattern; 4],
}

#[derive(Debug, thiserror::Error)]
enum InputParseError {
    #[error("Signal pattern error at pattern #{0}: {1}")]
    SignalPattern(u8, SignalPatternParseError),

    #[error("Missing separator")]
    MissingSeparator,

    #[error("Invalid number of patterns before the separator. Expected 10, found {0}")]
    CountBefore(u8),

    #[error("Invalid number of patterns after the separator. Expected 4, found {0}")]
    CountAfter(u8),
}

impl FromStr for Input {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InputParseError::*;

        let (before, after) = s.split_once(" | ").ok_or(MissingSeparator)?;

        let mut before = before.split_whitespace();
        let b0 = before
            .next()
            .ok_or(CountBefore(0))?
            .parse()
            .map_err(|err| SignalPattern(0, err))?;
        let b1 = before
            .next()
            .ok_or(CountBefore(1))?
            .parse()
            .map_err(|err| SignalPattern(1, err))?;
        let b2 = before
            .next()
            .ok_or(CountBefore(2))?
            .parse()
            .map_err(|err| SignalPattern(2, err))?;
        let b3 = before
            .next()
            .ok_or(CountBefore(3))?
            .parse()
            .map_err(|err| SignalPattern(3, err))?;
        let b4 = before
            .next()
            .ok_or(CountBefore(4))?
            .parse()
            .map_err(|err| SignalPattern(4, err))?;
        let b5 = before
            .next()
            .ok_or(CountBefore(5))?
            .parse()
            .map_err(|err| SignalPattern(5, err))?;
        let b6 = before
            .next()
            .ok_or(CountBefore(6))?
            .parse()
            .map_err(|err| SignalPattern(6, err))?;
        let b7 = before
            .next()
            .ok_or(CountBefore(7))?
            .parse()
            .map_err(|err| SignalPattern(7, err))?;
        let b8 = before
            .next()
            .ok_or(CountBefore(8))?
            .parse()
            .map_err(|err| SignalPattern(8, err))?;
        let b9 = before
            .next()
            .ok_or(CountBefore(9))?
            .parse()
            .map_err(|err| SignalPattern(9, err))?;
        if before.next().is_some() {
            return Err(CountBefore(11));
        }

        let unique_patterns = [b0, b1, b2, b3, b4, b5, b6, b7, b8, b9];

        let mut after = after.split_whitespace();
        let a0 = after
            .next()
            .ok_or(CountAfter(0))?
            .parse()
            .map_err(|err| SignalPattern(0, err))?;
        let a1 = after
            .next()
            .ok_or(CountAfter(1))?
            .parse()
            .map_err(|err| SignalPattern(1, err))?;
        let a2 = after
            .next()
            .ok_or(CountAfter(2))?
            .parse()
            .map_err(|err| SignalPattern(2, err))?;
        let a3 = after
            .next()
            .ok_or(CountAfter(3))?
            .parse()
            .map_err(|err| SignalPattern(3, err))?;
        if after.next().is_some() {
            return Err(CountBefore(5));
        }

        let output = [a0, a1, a2, a3];

        Ok(Input {
            unique_patterns,
            output,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe";
        let input = input.parse::<Input>().expect("successful parse");
        assert!(input.unique_patterns[0].contains(SignalWire::B));
        assert!(input.unique_patterns[0].contains(SignalWire::E));
        assert!(!input.unique_patterns[0].contains(SignalWire::A));

        assert!(input.output[1].contains(SignalWire::C));
        assert!(input.output[1].contains(SignalWire::E));
        assert!(!input.output[1].contains(SignalWire::A));
    }

    #[test]
    fn test_signal_pattern() {
        let input = "fdgacbe".parse::<SignalPattern>().unwrap();
        assert!(input.is_eight());
        assert!(input.is_counted_in_part_a());

        let input = "cefdb".parse::<SignalPattern>().unwrap();
        assert!(!input.is_counted_in_part_a());

        let input = "gcbe".parse::<SignalPattern>().unwrap();
        assert!(input.is_four());
        assert!(input.is_counted_in_part_a());

        let input = "cgb".parse::<SignalPattern>().unwrap();
        assert!(input.is_seven());
        assert!(input.is_counted_in_part_a());

        let input = "gc".parse::<SignalPattern>().unwrap();
        assert!(input.is_one());
        assert!(input.is_counted_in_part_a());
    }

    #[test]
    fn part_a() -> Result<()> {
        let input = include_str!("example.txt");
        let input = parse(input)?;
        let count = solve_a(&input);
        assert_eq!(count, 26);
        Ok(())
    }
}
