pub struct BitParser<'a> {
    input: &'a str,
    bit: u8,
}

impl<'a> BitParser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, bit: 0 }
    }

    fn consume_bit(&mut self) -> Result<Option<u8>, Error> {
        if self.input.is_empty() {
            return Ok(None);
        }
        let char = &self.input[0..1];

        let r = match self.bit {
            0 => match char {
                "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" => Ok(Some(0)),
                "8" | "9" | "A" | "B" | "C" | "D" | "E" | "F" => Ok(Some(1)),
                _ => Err(Error::InvalidCharacter(char.to_string())),
            },
            1 => match char {
                "0" | "1" | "2" | "3" | "8" | "9" | "A" | "B" => Ok(Some(0)),
                "4" | "5" | "6" | "7" | "C" | "D" | "E" | "F" => Ok(Some(1)),
                _ => Err(Error::InvalidCharacter(char.to_string())),
            },
            2 => match char {
                "0" | "1" | "4" | "5" | "8" | "9" | "C" | "D" => Ok(Some(0)),
                "2" | "3" | "6" | "7" | "A" | "B" | "E" | "F" => Ok(Some(1)),
                _ => Err(Error::InvalidCharacter(char.to_string())),
            },
            3 => match char {
                "0" | "2" | "4" | "6" | "8" | "A" | "C" | "E" => Ok(Some(0)),
                "1" | "3" | "5" | "7" | "9" | "B" | "D" | "F" => Ok(Some(1)),
                _ => Err(Error::InvalidCharacter(char.to_string())),
            },
            _ => unreachable!(),
        };

        self.bit += 1;
        if self.bit == 4 {
            self.bit = 0;
            self.input = &self.input[1..];
        }

        r
    }

    pub fn consume_bits(&mut self, bits: u8) -> Result<Option<u8>, Error> {
        if bits > 8 {
            return Err(Error::TooManyBitsRequested);
        }

        let mut value = 0;
        for _ in 0..bits {
            let bit = self.consume_bit()?;
            if let Some(bit) = bit {
                value = (value << 1) | bit;
            } else {
                return Ok(None);
            }
        }

        Ok(Some(value))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Too many bits requested")]
    TooManyBitsRequested,

    #[error("Invalid character: {0}")]
    InvalidCharacter(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

    #[test]
    fn test_bit_parser_single_bits() -> Result<()> {
        let input = "D2FE28";
        let mut bit_parser = BitParser::new(input);
        let mut actual = Vec::new();
        loop {
            let v = bit_parser.consume_bits(1)?;
            if let Some(v) = v {
                actual.push(v.to_string());
            } else {
                break;
            }
        }
        let expected = "110100101111111000101000";
        assert_eq!(actual.join(""), expected);

        Ok(())
    }

    #[test]
    fn test_bit_parser_logical_bits() -> Result<()> {
        let input = "D2FE28";

        // 110100101111111000101000
        // VVVTTTAAAAABBBBBCCCCC
        let mut bit_parser = BitParser::new(input);
        assert_eq!(bit_parser.consume_bits(3)?, Some(6), "V");
        assert_eq!(bit_parser.consume_bits(3)?, Some(4), "T");
        assert_eq!(bit_parser.consume_bits(1)?, Some(1), "First bit of A");
        assert_eq!(
            bit_parser.consume_bits(4)?,
            Some(0b0111),
            "Next four bits of A"
        );
        assert_eq!(bit_parser.consume_bits(1)?, Some(1), "First bit of B");
        assert_eq!(
            bit_parser.consume_bits(4)?,
            Some(0b1110),
            "Next four bits of B"
        );
        assert_eq!(bit_parser.consume_bits(5)?, Some(0b00101), "C");
        assert_eq!(bit_parser.consume_bits(4)?, None);
        Ok(())
    }
}
