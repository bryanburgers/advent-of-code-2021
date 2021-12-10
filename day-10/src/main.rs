fn main() {
    let input = include_str!("input.txt");
    let result = solve_a(input);
    println!("{}", result);
    let result = solve_b(input);
    println!("{}", result);
}

fn solve_a(input: &str) -> usize {
    let mut score = 0;
    for line in input.lines() {
        let mut parser = Parser::default();
        match parser.parse(line) {
            Err(ParseError::InvalidBracket { found, .. }) => {
                score += found.mismatched_score();
            }
            _ => {}
        }
    }
    score
}

fn solve_b(input: &str) -> usize {
    let mut scores = Vec::new();
    for line in input.lines() {
        let mut parser = Parser::default();
        match parser.parse(line) {
            Err(ParseError::UnexpectedEof { .. }) => {
                scores.push(parser.autocomplete());
            }
            _ => {}
        }
    }
    scores.sort();
    scores[scores.len() / 2]
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Token {
    Open(Bracket),
    Close(Bracket),
    Other(char),
    Eof,
}

impl From<char> for Token {
    fn from(c: char) -> Self {
        match c {
            '(' => Token::Open(Bracket::Round),
            '{' => Token::Open(Bracket::Curly),
            '[' => Token::Open(Bracket::Square),
            '<' => Token::Open(Bracket::Angle),
            ')' => Token::Close(Bracket::Round),
            '}' => Token::Close(Bracket::Curly),
            ']' => Token::Close(Bracket::Square),
            '>' => Token::Close(Bracket::Angle),
            c => Token::Other(c),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Bracket {
    Round,
    Square,
    Curly,
    Angle,
}

impl Bracket {
    fn mismatched_score(&self) -> usize {
        match self {
            Self::Round => 3,
            Self::Square => 57,
            Self::Curly => 1197,
            Self::Angle => 25137,
        }
    }

    fn unconsumed_score(&self) -> usize {
        match self {
            Self::Round => 1,
            Self::Square => 2,
            Self::Curly => 3,
            Self::Angle => 4,
        }
    }
}

#[derive(Debug, thiserror::Error, Eq, PartialEq)]
enum ParseError {
    #[error("Invalid backet type. Expected {expected:?}, found {found:?}")]
    InvalidBracket { expected: Bracket, found: Bracket },

    #[error("Unexpected EOF. Expecting close bracket of type {expected:?}")]
    UnexpectedEof { expected: Bracket },

    #[error("Close bracket with no opening bracket: {found:?}")]
    UnexpectedCloseBracket { found: Bracket },

    #[error("Unexpected char '{char}'.")]
    UnexpectedChar { char: char },
}

#[derive(Debug, Default)]
struct Parser {
    stack: Vec<Bracket>,
}

impl Parser {
    fn parse(&mut self, input: &str) -> Result<(), ParseError> {
        for (idx, c) in input.char_indices() {
            let token = Token::from(c);
            self.consume(idx, token)?;
        }
        self.consume(input.len(), Token::Eof)?;
        Ok(())
    }

    fn consume(&mut self, _idx: usize, token: Token) -> Result<(), ParseError> {
        match token {
            Token::Open(open_bracket) => {
                self.stack.push(open_bracket);
                Ok(())
            }
            Token::Close(close_bracket) => match self.stack.pop() {
                None => Err(ParseError::UnexpectedCloseBracket {
                    found: close_bracket,
                }),
                Some(open_bracket) if open_bracket != close_bracket => {
                    Err(ParseError::InvalidBracket {
                        found: close_bracket,
                        expected: open_bracket,
                    })
                }
                Some(_) => Ok(()),
            },
            Token::Other(char) => Err(ParseError::UnexpectedChar { char }),
            Token::Eof => match self.stack.last() {
                None => Ok(()),
                Some(bracket) => Err(ParseError::UnexpectedEof { expected: *bracket }),
            },
        }
    }

    fn autocomplete(&mut self) -> usize {
        let mut total = 0;
        while let Some(bracket) = self.stack.pop() {
            total = total * 5 + bracket.unconsumed_score()
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert!(Parser::default().parse("([])").is_ok());
        assert!(Parser::default().parse("{()()()}").is_ok());
        assert!(Parser::default().parse("<([{}])>").is_ok());
        assert!(Parser::default().parse("[<>({}){}[([])<>]]").is_ok());
        assert!(Parser::default().parse("(((((((((())))))))))").is_ok());

        assert_eq!(
            Parser::default().parse("(]"),
            Err(ParseError::InvalidBracket {
                expected: Bracket::Round,
                found: Bracket::Square
            })
        );
    }

    #[test]
    fn test_solve_a() {
        let input = include_str!("example.txt");
        let result = solve_a(input);
        assert_eq!(result, 26397);
    }

    #[test]
    fn test_autocomplete() {
        let mut parser = Parser::default();
        let result = parser.parse("[({(<(())[]>[[{[]{<()<>>");
        assert_eq!(
            result,
            Err(ParseError::UnexpectedEof {
                expected: Bracket::Curly
            })
        );
        assert_eq!(parser.autocomplete(), 288957);
    }

    #[test]
    fn test_solve_b() {
        let input = include_str!("example.txt");
        let result = solve_b(input);
        assert_eq!(result, 288957);
    }
}
