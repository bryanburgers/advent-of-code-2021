type Result<T, E = Box<dyn std::error::Error + Send + Sync>> = std::result::Result<T, E>;

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    let parsed = parse(input)?;
    let increases = count_increases(&parsed);
    println!("{}", increases);
    let increases = count_increase_windows(&parsed);
    println!("{}", increases);

    Ok(())
}

fn parse(input: &str) -> Result<Vec<u64>> {
    input
        .lines()
        .map(|line| line.parse::<u64>().map_err(|err| err.into()))
        .collect::<Result<_>>()
}

fn count_increases(inputs: &[u64]) -> usize {
    let windows = inputs.windows(2);
    let mut count = 0;
    for window in windows {
        let first = window[0];
        let second = window[1];
        if second > first {
            count += 1;
        }
    }
    count
}

fn count_increase_windows(inputs: &[u64]) -> usize {
    let windows = inputs.windows(3);
    let mut count = 0;
    let mut prev: Option<u64> = None;

    for window in windows {
        let sum = window.iter().copied().sum();
        if let Some(prev_val) = prev {
            if sum > prev_val {
                count += 1;
            }
        }
        prev = Some(sum);
    }

    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() -> Result<()> {
        let str = include_str!("example.txt");
        let parsed = parse(str)?;
        let increases = count_increases(&parsed);

        assert_eq!(increases, 7);

        Ok(())
    }

    #[test]
    fn test_2() -> Result<()> {
        let str = include_str!("example.txt");
        let parsed = parse(str)?;
        let increases = count_increase_windows(&parsed);

        assert_eq!(increases, 5);

        Ok(())
    }
}
