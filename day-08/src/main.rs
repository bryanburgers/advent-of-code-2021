use std::{collections::HashSet, str::FromStr};

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    let input = parse(input)?;
    let count = solve_a(&input);
    println!("{}", count);
    let sum = solve_b(&input);
    println!("{}", sum);

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

fn solve_b(input: &[Input]) -> usize {
    let mut sum = 0;
    for item in input {
        sum += item.deduce_output();
    }
    sum
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
    // fn wires(&self) -> Vec<SignalWire> {
    //     self.signals.iter().copied().collect()
    // }
    // fn not_wires(&self) -> Vec<SignalWire> {
    //     let wires = vec![
    //         SignalWire::A,
    //         SignalWire::B,
    //         SignalWire::C,
    //         SignalWire::D,
    //         SignalWire::E,
    //         SignalWire::F,
    //         SignalWire::G,
    //     ];

    //     wires
    //         .into_iter()
    //         .filter(|w| self.signals.contains(w))
    //         .collect()
    // }
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

impl Input {
    fn find_one<F>(&self, f: F) -> &SignalPattern
    where
        F: Fn(&SignalPattern) -> bool,
    {
        for pattern in &self.unique_patterns {
            if f(pattern) {
                return pattern;
            }
        }
        panic!("Not found");
    }
    fn find_many<F>(&self, f: F) -> Vec<&SignalPattern>
    where
        F: Fn(&SignalPattern) -> bool,
    {
        self.unique_patterns
            .as_slice()
            .iter()
            .filter(|v| f(*v))
            .collect()
    }

    fn determine_segments(&self) -> SegmentMap {
        let all_wires = HashSet::from([
            SignalWire::A,
            SignalWire::B,
            SignalWire::C,
            SignalWire::D,
            SignalWire::E,
            SignalWire::F,
            SignalWire::G,
        ]);

        let mut top = all_wires.clone();
        let mut top_left = all_wires.clone();
        let mut top_right = all_wires.clone();
        let mut middle = all_wires.clone();
        let mut bottom_left = all_wires.clone();
        let mut bottom_right = all_wires.clone();
        let mut bottom = all_wires.clone();

        let signal_pattern_one = self.find_one(|pattern| pattern.is_one());
        let signal_pattern_seven = self.find_one(|pattern| pattern.is_seven());
        let signal_pattern_four = self.find_one(|pattern| pattern.is_four());

        // We know 1, so apply it.
        for wire in &all_wires {
            if signal_pattern_one.contains(*wire) {
                top.remove(wire);
                top_left.remove(wire);
                middle.remove(wire);
                bottom_left.remove(wire);
                bottom.remove(wire);
            } else {
                top_right.remove(wire);
                bottom_right.remove(wire);
            }
        }

        // At this point, the right two segments should have 2 options
        assert_eq!(
            top_right.len(),
            2,
            "at this point, the right two segments should have 2 options (top)"
        );
        assert_eq!(
            bottom_right.len(),
            2,
            "at this point, the right two segments should have 2 options (bottom)"
        );

        // We know 7, so apply it.
        for wire in &all_wires {
            if signal_pattern_seven.contains(*wire) {
                top_left.remove(wire);
                middle.remove(wire);
                bottom_left.remove(wire);
                bottom.remove(wire);
            } else {
                top.remove(wire);
                top_right.remove(wire);
                bottom_right.remove(wire);
            }
        }

        // At this point, the top segment should be unique
        assert_eq!(
            top.len(),
            1,
            "At this point, the top segment should be unique"
        );
        // And the right two segments should still be have 2 options
        assert_eq!(
            top_right.len(),
            2,
            "and the right two segments should still have 2 options (top)"
        );
        assert_eq!(
            bottom_right.len(),
            2,
            "and the right two segments should still have 2 options (bottom)"
        );

        // We know 4, so apply it.
        for wire in &all_wires {
            if signal_pattern_four.contains(*wire) {
                top.remove(wire);
                bottom_left.remove(wire);
                bottom.remove(wire);
            } else {
                top_left.remove(wire);
                top_right.remove(wire);
                middle.remove(wire);
                bottom_right.remove(wire);
            }
        }

        assert_eq!(
            bottom_left.len(),
            2,
            "At this point, there should only be two options remaining for the bottom left segment"
        );

        // We can use this to determine which of the signal patterns is the 2.
        // - There are three signal patterns with 5 signals set: 2, 3, 5
        // - Both the 3 and the 5 do not have the bottom left segment set; the 2 does.
        let signal_pattern_235 = self.find_many(|pattern| pattern.signals_set() == 5);
        let signal_wire_0 = *bottom_left.iter().nth(0).unwrap();
        let signal_wire_1 = *bottom_left.iter().nth(1).unwrap();
        let signal_wire_0_matches = signal_pattern_235
            .iter()
            .filter(|pattern| pattern.contains(signal_wire_0))
            .cloned()
            .collect::<Vec<_>>();
        let signal_wire_1_matches = signal_pattern_235
            .iter()
            .filter(|pattern| pattern.contains(signal_wire_1))
            .cloned()
            .collect::<Vec<_>>();

        assert!(
            (signal_wire_0_matches.len() == 1 && signal_wire_1_matches.len() == 3)
                || (signal_wire_0_matches.len() == 3 && signal_wire_1_matches.len() == 1),
            "One of these should have 1 match, one should have 3"
        );

        let signal_wire_bottom_left: SignalWire;
        let signal_wire_not_bottom_left: SignalWire;
        let signal_pattern_two: &SignalPattern;
        if signal_wire_0_matches.len() == 1 {
            signal_wire_bottom_left = signal_wire_0;
            signal_wire_not_bottom_left = signal_wire_1;
            signal_pattern_two = signal_wire_0_matches.into_iter().next().unwrap();
        } else {
            signal_wire_bottom_left = signal_wire_1;
            signal_wire_not_bottom_left = signal_wire_0;
            signal_pattern_two = signal_wire_1_matches.into_iter().next().unwrap();
        };

        // And now apply the 2
        for wire in &all_wires {
            if signal_pattern_two.contains(*wire) {
                top_left.remove(wire);
                bottom_right.remove(wire);
            } else {
                top.remove(wire);
                top_right.remove(wire);
                middle.remove(wire);
                bottom_left.remove(wire);
                bottom.remove(wire);
            }
        }

        // And apply which wire we know is bottom left
        bottom_left.remove(&signal_wire_not_bottom_left);
        bottom.remove(&signal_wire_bottom_left);

        assert_eq!(top.len(), 1);
        assert_eq!(top_left.len(), 1);
        assert_eq!(top_right.len(), 1);
        assert_eq!(middle.len(), 1);
        assert_eq!(bottom_left.len(), 1);
        assert_eq!(bottom_right.len(), 1);
        assert_eq!(bottom.len(), 1);

        SegmentMap {
            top: top.into_iter().next().unwrap(),
            top_left: top_left.into_iter().next().unwrap(),
            top_right: top_right.into_iter().next().unwrap(),
            middle: middle.into_iter().next().unwrap(),
            bottom_left: bottom_left.into_iter().next().unwrap(),
            bottom_right: bottom_right.into_iter().next().unwrap(),
            bottom: bottom.into_iter().next().unwrap(),
        }
    }

    fn deduce_output(&self) -> usize {
        let signal_map = self.determine_segments();

        let mut val = 0;
        val += signal_map.to_number(&self.output[0]).unwrap() * 1000;
        val += signal_map.to_number(&self.output[1]).unwrap() * 100;
        val += signal_map.to_number(&self.output[2]).unwrap() * 10;
        val += signal_map.to_number(&self.output[3]).unwrap() * 1;
        val
    }
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

#[derive(Debug)]
struct SegmentMap {
    top: SignalWire,
    top_left: SignalWire,
    top_right: SignalWire,
    middle: SignalWire,
    bottom_left: SignalWire,
    bottom_right: SignalWire,
    bottom: SignalWire,
}

impl SegmentMap {
    fn to_number(&self, pattern: &SignalPattern) -> Option<usize> {
        let all = HashSet::from([
            self.top,
            self.top_left,
            self.top_right,
            self.middle,
            self.bottom_left,
            self.bottom_right,
            self.bottom,
        ]);
        let numbers = [
            /* 0 */
            HashSet::from([
                self.top,
                self.top_left,
                self.top_right,
                self.bottom_left,
                self.bottom_right,
                self.bottom,
            ]),
            /* 1 */
            HashSet::from([self.top_right, self.bottom_right]),
            /* 2 */
            HashSet::from([
                self.top,
                self.top_right,
                self.middle,
                self.bottom_left,
                self.bottom,
            ]),
            /* 3 */
            HashSet::from([
                self.top,
                self.top_right,
                self.middle,
                self.bottom_right,
                self.bottom,
            ]),
            /* 4 */
            HashSet::from([
                self.top_left,
                self.top_right,
                self.middle,
                self.bottom_right,
            ]),
            /* 5 */
            HashSet::from([
                self.top,
                self.top_left,
                self.middle,
                self.bottom_right,
                self.bottom,
            ]),
            /* 6 */
            HashSet::from([
                self.top,
                self.top_left,
                self.middle,
                self.bottom_left,
                self.bottom_right,
                self.bottom,
            ]),
            /* 7 */
            HashSet::from([self.top, self.top_right, self.bottom_right]),
            /* 8 */
            HashSet::from([
                self.top,
                self.top_left,
                self.top_right,
                self.middle,
                self.bottom_left,
                self.bottom_right,
                self.bottom,
            ]),
            /* 9 */
            HashSet::from([
                self.top,
                self.top_left,
                self.top_right,
                self.middle,
                self.bottom_right,
                self.bottom,
            ]),
        ];

        'outer: for i in 0..=9 {
            let positive = &numbers[i];
            let negative = all.difference(positive);
            for v in positive {
                if !pattern.contains(*v) {
                    continue 'outer;
                }
            }
            for v in negative {
                if pattern.contains(*v) {
                    continue 'outer;
                }
            }
            return Some(i);
        }

        None
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

    #[test]
    fn test_determine_segments() {
        let input =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        let input = input.parse::<Input>().expect("Valid parse");
        let segment_map = input.determine_segments();

        assert_eq!(segment_map.top, SignalWire::D);
        assert_eq!(segment_map.top_left, SignalWire::E);
        assert_eq!(segment_map.top_right, SignalWire::A);
        assert_eq!(segment_map.middle, SignalWire::F);
        assert_eq!(segment_map.bottom_left, SignalWire::G);
        assert_eq!(segment_map.bottom_right, SignalWire::B);
        assert_eq!(segment_map.bottom, SignalWire::C);

        assert_eq!(input.deduce_output(), 5353);
    }

    #[test]
    fn part_b() -> Result<()> {
        let input = include_str!("example.txt");
        let input = parse(input)?;
        let sum = solve_b(&input);
        assert_eq!(sum, 61229);
        Ok(())
    }
}
