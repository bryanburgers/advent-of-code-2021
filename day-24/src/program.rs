use std::{fmt::Display, str::FromStr};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Variable {
    W,
    X,
    Y,
    Z,
}

impl FromStr for Variable {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "w" => Ok(Self::W),
            "x" => Ok(Self::X),
            "y" => Ok(Self::Y),
            "z" => Ok(Self::Z),
            _ => Err(format!("Expected 'w', 'x', 'y', or 'z'. Got '{}'", s)),
        }
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::W => f.write_str("w"),
            Self::X => f.write_str("x"),
            Self::Y => f.write_str("y"),
            Self::Z => f.write_str("z"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Parameter {
    Variable(Variable),
    Number(i32),
}

impl FromStr for Parameter {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let as_var = s.parse::<Variable>();
        let as_num = s.parse::<i32>();
        match (as_var, as_num) {
            (Ok(var), _) => Ok(Self::Variable(var)),
            (_, Ok(num)) => Ok(Self::Number(num)),
            _ => Err(format!("Expected variable or number. Got '{}'", s)),
        }
    }
}

impl Display for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Variable(var) => write!(f, "{}", var),
            Self::Number(n) => write!(f, "{}", n),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Instruction {
    Inp(Variable),
    Add(Variable, Parameter),
    Mul(Variable, Parameter),
    Div(Variable, Parameter),
    Mod(Variable, Parameter),
    Eql(Variable, Parameter),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_variable_and_parameter<'a, F>(
            instruction: &'a str,
            split: &mut std::str::SplitWhitespace<'a>,
            f: F,
        ) -> Result<Instruction, String>
        where
            F: Fn(Variable, Parameter) -> Instruction,
        {
            let variable = split.next().ok_or_else(|| {
                format!(
                    "Variable for '{}' invalid: no variable provided",
                    instruction
                )
            })?;
            let variable = variable
                .parse()
                .map_err(|err| format!("Variable for '{}' invalid: {}", instruction, err))?;
            let parameter = split.next().ok_or_else(|| {
                format!("Parameter for '{}' invalid: no input provided", instruction)
            })?;
            let parameter = parameter
                .parse()
                .map_err(|err| format!("Parameter for '{}' invalid: {}", instruction, err))?;

            if let Some(next) = split.next() {
                return Err(format!(
                    "Instruction '{}' received third parameter '{}' but only expected two",
                    instruction, next
                ));
            }

            Ok(f(variable, parameter))
        }

        let mut split = s.split_whitespace();
        let first = split
            .next()
            .ok_or_else(|| String::from("Expected instruction, got EOL"))?;
        match first {
            "inp" => {
                let variable = split.next().ok_or_else(|| {
                    String::from("Variable for 'inp' invalid: no variable provided")
                })?;
                let variable = variable
                    .parse()
                    .map_err(|err| format!("Variable for 'inp' invalid: {}", err))?;

                if let Some(next) = split.next() {
                    return Err(format!(
                        "Instruction 'inp' received second parameter '{}' but only expected one",
                        next
                    ));
                }
                Ok(Self::Inp(variable))
            }
            "add" => parse_variable_and_parameter("add", &mut split, Instruction::Add),
            "mul" => parse_variable_and_parameter("mul", &mut split, Instruction::Mul),
            "div" => parse_variable_and_parameter("div", &mut split, Instruction::Div),
            "mod" => parse_variable_and_parameter("mod", &mut split, Instruction::Mod),
            "eql" => parse_variable_and_parameter("eql", &mut split, Instruction::Eql),
            _ => Err(format!("Unknown instruction '{}' in line '{}'", first, s)),
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Inp(var) => write!(f, "inp {}", var),
            Instruction::Add(var1, var2) => write!(f, "add {} {}", var1, var2),
            Instruction::Mul(var1, var2) => write!(f, "mul {} {}", var1, var2),
            Instruction::Div(var1, var2) => write!(f, "div {} {}", var1, var2),
            Instruction::Mod(var1, var2) => write!(f, "mod {} {}", var1, var2),
            Instruction::Eql(var1, var2) => write!(f, "eql {} {}", var1, var2),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Program(pub Vec<Instruction>);

impl Program {
    pub fn instructions(&self) -> &[Instruction] {
        &self.0
    }
}

impl FromStr for Program {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut instructions = Vec::new();
        for (idx, line) in s.lines().enumerate() {
            let instruction = line
                .parse()
                .map_err(|err| format!("Failed to parse on line {}: {}", idx + 1, err))?;
            instructions.push(instruction);
        }
        Ok(Self(instructions))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_example() {
        let example = include_str!("example.txt");
        example
            .parse::<Program>()
            .expect("Should parse successfully");
    }

    #[test]
    fn test_parse_input() {
        let example = include_str!("input.txt");
        example
            .parse::<Program>()
            .expect("Should parse successfully");
    }
}
