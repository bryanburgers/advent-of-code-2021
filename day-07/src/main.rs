use std::str::FromStr;

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

fn main() -> Result<()> {
    let crabs = parse(include_str!("input.txt"))?;
    let result_a = solve_a(&crabs);
    println!("{}", result_a);
    let result_b = solve_b(&crabs);
    println!("{}", result_b);

    Ok(())
}

fn parse(input: &str) -> Result<Vec<Crab>> {
    input
        .split(',')
        .map(|part| part.parse().map_err(Into::into))
        .collect::<Result<_>>()
}

fn solve_a(crabs: &[Crab]) -> i64 {
    let mut min_position = i16::MAX;
    let mut max_position = i16::MIN;
    for crab in crabs {
        if crab.horizontal_position < min_position {
            min_position = crab.horizontal_position;
        }
        if crab.horizontal_position > max_position {
            max_position = crab.horizontal_position;
        }
    }

    let mut least_fuel_used = i64::MAX;
    for target_position in min_position..=max_position {
        let mut fuel_used = 0;

        for crab in crabs {
            fuel_used += crab.fuel_used_for_position(target_position);
        }
        if fuel_used < least_fuel_used {
            least_fuel_used = fuel_used;
        }
    }

    least_fuel_used
}

fn solve_b(crabs: &[Crab]) -> i64 {
    let mut min_position = i16::MAX;
    let mut max_position = i16::MIN;
    for crab in crabs {
        if crab.horizontal_position < min_position {
            min_position = crab.horizontal_position;
        }
        if crab.horizontal_position > max_position {
            max_position = crab.horizontal_position;
        }
    }

    let mut least_fuel_used = i64::MAX;
    for target_position in min_position..=max_position {
        let mut fuel_used = 0;

        for crab in crabs {
            fuel_used += crab.fuel_used_for_position_2(target_position);
        }
        if fuel_used < least_fuel_used {
            least_fuel_used = fuel_used;
        }
    }

    least_fuel_used
}

#[derive(Debug, Copy, Clone)]
struct Crab {
    horizontal_position: i16,
}

impl Crab {
    fn fuel_used_for_position(&self, x: i16) -> i64 {
        (self.horizontal_position - x).abs().into()
    }

    fn fuel_used_for_position_2(&self, x: i16) -> i64 {
        let difference: i64 = (self.horizontal_position - x).abs().into();
        (difference * (difference + 1)) / 2
    }
}

impl FromStr for Crab {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let horizontal_position = s.parse().map_err(|_| "Failed to parse number")?;
        Ok(Crab {
            horizontal_position,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuel_used() -> Result<()> {
        let crab = Crab {
            horizontal_position: 16,
        };
        assert_eq!(crab.fuel_used_for_position(16), 0);
        assert_eq!(crab.fuel_used_for_position(17), 1);
        assert_eq!(crab.fuel_used_for_position(15), 1);
        assert_eq!(crab.fuel_used_for_position(0), 16);
        Ok(())
    }

    #[test]
    fn test_1() -> Result<()> {
        let crabs = parse(include_str!("example.txt"))?;
        let result = solve_a(&crabs);
        assert_eq!(result, 37);

        Ok(())
    }

    #[test]
    fn fuel_used_2() -> Result<()> {
        let crab = Crab {
            horizontal_position: 16,
        };
        assert_eq!(crab.fuel_used_for_position_2(16), 0);
        assert_eq!(crab.fuel_used_for_position_2(17), 1);
        assert_eq!(crab.fuel_used_for_position_2(18), 1 + 2);
        assert_eq!(crab.fuel_used_for_position_2(19), 1 + 2 + 3);
        assert_eq!(crab.fuel_used_for_position_2(20), 1 + 2 + 3 + 4);
        assert_eq!(crab.fuel_used_for_position_2(15), 1);
        assert_eq!(crab.fuel_used_for_position_2(14), 1 + 2);
        assert_eq!(crab.fuel_used_for_position_2(13), 1 + 2 + 3);
        assert_eq!(crab.fuel_used_for_position_2(12), 1 + 2 + 3 + 4);
        Ok(())
    }

    #[test]
    fn test_2() -> Result<()> {
        let crabs = parse(include_str!("example.txt"))?;
        let result = solve_b(&crabs);
        assert_eq!(result, 168);

        Ok(())
    }
}
