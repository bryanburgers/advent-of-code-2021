use std::str::FromStr;

type Result<T, E = Box<dyn std::error::Error + Send + Sync>> = std::result::Result<T, E>;

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    let parsed = parse(input)?;
    let mut position = Position::default();
    for command in &parsed {
        position.apply(command);
    }
    println!("{} ({:?})", position.horizontal_position * position.depth, position);

    Ok(())
}

fn parse(input: &str) -> Result<Vec<Command>> {
    input
        .lines()
        .map(|line| line.parse::<Command>().map_err(|err| err.into()))
        .collect::<Result<_>>()
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Command {
    Forward(i64),
    Down(i64),
    Up(i64),
}

impl FromStr for Command {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (command, distance) = s.split_once(" ").ok_or("no space found")?;
        let distance = distance.parse::<i64>().map_err(|_| "distance was not a number")?;
        match command {
            "forward" => Ok(Command::Forward(distance)),
            "down" => Ok(Command::Down(distance)),
            "up" => Ok(Command::Up(distance)),
            _ => Err("unrecognized command"),
        }
    }
}

#[derive(Default, Copy, Clone, Debug, Eq, PartialEq)]
struct Position {
    depth: i64,
    horizontal_position: i64,
}

impl Position {
    fn apply(&mut self, command: &Command) {
        match command {
            Command::Down(distance) => { self.depth += distance; }
            Command::Up(distance) => { self.depth -= distance; }
            Command::Forward(distance) => { self.horizontal_position += distance; }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_command() {
        assert_eq!("forward 4".parse::<Command>(), Ok(Command::Forward(4)));
        assert_eq!("down 4".parse::<Command>(), Ok(Command::Down(4)));
        assert_eq!("up 4".parse::<Command>(), Ok(Command::Up(4)));
        assert!("other 4".parse::<Command>().is_err());
    }

    #[test]
    fn apply_command() {
        let mut position = Position::default();
        assert_eq!(position, Position { depth: 0, horizontal_position: 0 });

        position.apply(&Command::Forward(6));
        assert_eq!(position, Position { depth: 0, horizontal_position: 6 });

        position.apply(&Command::Down(5));
        assert_eq!(position, Position { depth: 5, horizontal_position: 6 });

        position.apply(&Command::Up(3));
        assert_eq!(position, Position { depth: 2, horizontal_position: 6 });
    }

    #[test]
    fn test_1() -> Result<()> {
        let str = include_str!("example.txt");
        let parsed = parse(str)?;
        let mut position = Position::default();
        for command in &parsed {
            position.apply(command);
        }
        assert_eq!(position, Position { depth: 10, horizontal_position: 15 });

        Ok(())
    }
}
