use crate::bit_parser::BitParser;

pub struct PacketParser<'a> {
    bit_parser: BitParser<'a>,
}

impl<'a> PacketParser<'a> {
    pub fn new(input: &'a str) -> Self {
        let bit_parser = BitParser::new(input);
        Self { bit_parser }
    }

    pub fn next(&mut self) -> Result<Option<Packet>, Error> {
        let version = if let Some(version) = self.bit_parser.consume_bits(3)? {
            Version(version)
        } else {
            return Ok(None);
        };

        let type_id = if let Some(type_id) = self.bit_parser.consume_bits(3)? {
            TypeId(type_id)
        } else {
            return Ok(None);
        };

        let payload = if type_id.is_literal() {
            self.parse_literal_payload()
        } else {
            self.parse_operator_payload()
        };

        let payload = match payload {
            Ok(payload) => payload,
            Err(Error::UnexpectedEof) => return Ok(None),
            Err(err) => return Err(err),
        };

        let packet = Packet {
            version,
            type_id,
            payload,
        };

        Ok(Some(packet))
    }

    fn parse_literal_payload(&mut self) -> Result<Payload, Error> {
        let mut value = 0;
        let mut chunks = 0;

        loop {
            let continuation = self
                .bit_parser
                .consume_bits(1)?
                .ok_or(Error::UnexpectedEof)?;
            let v = self
                .bit_parser
                .consume_bits(4)?
                .ok_or(Error::UnexpectedEof)?;
            chunks += 1;
            value = value << 4 | v as u64;
            if continuation == 0 {
                break;
            }
        }

        Ok(Payload::Literal { chunks, value })
    }

    fn parse_operator_payload(&mut self) -> Result<Payload, Error> {
        let t = self
            .bit_parser
            .consume_bits(1)?
            .ok_or(Error::UnexpectedEof)?;
        if t == 0 {
            self.parse_bits_operator_payload()
        } else {
            self.parse_packets_operator_payload()
        }
    }

    fn parse_bits_operator_payload(&mut self) -> Result<Payload, Error> {
        let high_order = self
            .bit_parser
            .consume_bits(7)?
            .ok_or(Error::UnexpectedEof)? as u16;
        let low_order = self
            .bit_parser
            .consume_bits(8)?
            .ok_or(Error::UnexpectedEof)? as u16;
        let bits = high_order << 8 | low_order;
        Ok(Payload::OperatorBitLength(bits))
    }

    fn parse_packets_operator_payload(&mut self) -> Result<Payload, Error> {
        let high_order = self
            .bit_parser
            .consume_bits(3)?
            .ok_or(Error::UnexpectedEof)? as u16;
        let low_order = self
            .bit_parser
            .consume_bits(8)?
            .ok_or(Error::UnexpectedEof)? as u16;
        let packets = high_order << 8 | low_order;
        Ok(Payload::OperatorPacketLength(packets))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    BitParser(#[from] crate::bit_parser::Error),

    #[error("Not implemented")]
    NotImplemented,

    #[error("Unexpected end of input")]
    UnexpectedEof,
}

/// Version number of the packet
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Version(u8);

impl Version {
    pub fn into_u8(self) -> u8 {
        self.0
    }
}

/// Type ID of the packet
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct TypeId(u8);

impl TypeId {
    pub fn is_literal(&self) -> bool {
        self.0 == 4
    }

    pub fn is_operator(&self) -> bool {
        !self.is_literal()
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Payload {
    OperatorBitLength(u16),
    OperatorPacketLength(u16),
    Literal { chunks: u8, value: u64 },
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Packet {
    pub version: Version,
    pub type_id: TypeId,
    pub payload: Payload,
}

#[cfg(test)]
mod tests {
    use super::*;

    type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

    #[test]
    fn test_literal() -> Result<()> {
        let input = "D2FE28";
        let mut packet_parser = PacketParser::new(input);
        let packet = packet_parser.next()?.expect("Expected packet");
        assert_eq!(
            packet,
            Packet {
                version: Version(6),
                type_id: TypeId(4),
                payload: Payload::Literal {
                    chunks: 3,
                    value: 2021
                },
            }
        );
        let packet = packet_parser.next()?;
        assert_eq!(packet, None);
        Ok(())
    }

    #[test]
    fn test_example_2() -> Result<()> {
        let input = "38006F45291200";
        let mut packet_parser = PacketParser::new(input);
        let packet = packet_parser.next()?.expect("Expected packet");
        assert_eq!(
            packet,
            Packet {
                version: Version(1),
                type_id: TypeId(6),
                payload: Payload::OperatorBitLength(27),
            }
        );
        let packet = packet_parser.next()?.expect("Expected packet");
        assert_eq!(
            packet,
            Packet {
                version: Version(6),
                type_id: TypeId(4),
                payload: Payload::Literal {
                    chunks: 1,
                    value: 10,
                },
            }
        );
        let packet = packet_parser.next()?.expect("Expected packet");
        assert_eq!(
            packet,
            Packet {
                version: Version(2),
                type_id: TypeId(4),
                payload: Payload::Literal {
                    chunks: 2,
                    value: 20,
                },
            }
        );
        let packet = packet_parser.next()?;
        assert_eq!(packet, None);
        Ok(())
    }

    #[test]
    fn test_example_3() -> Result<()> {
        let input = "EE00D40C823060";
        let mut packet_parser = PacketParser::new(input);
        let packet = packet_parser.next()?.expect("Expected packet");
        assert_eq!(
            packet,
            Packet {
                version: Version(7),
                type_id: TypeId(3),
                payload: Payload::OperatorPacketLength(3),
            }
        );
        let packet = packet_parser.next()?.expect("Expected packet");
        assert_eq!(
            packet,
            Packet {
                version: Version(2),
                type_id: TypeId(4),
                payload: Payload::Literal {
                    chunks: 1,
                    value: 1,
                },
            }
        );
        let packet = packet_parser.next()?.expect("Expected packet");
        assert_eq!(
            packet,
            Packet {
                version: Version(4),
                type_id: TypeId(4),
                payload: Payload::Literal {
                    chunks: 1,
                    value: 2,
                },
            }
        );
        let packet = packet_parser.next()?.expect("Expected packet");
        assert_eq!(
            packet,
            Packet {
                version: Version(1),
                type_id: TypeId(4),
                payload: Payload::Literal {
                    chunks: 1,
                    value: 3,
                },
            }
        );
        let packet = packet_parser.next()?;
        assert_eq!(packet, None);
        Ok(())
    }
}
