use crate::packet::{Packet, PacketParser, Payload, TypeId};

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum AstNode {
    Sum(Vec<AstNode>),
    Product(Vec<AstNode>),
    Minimum(Vec<AstNode>),
    Maximum(Vec<AstNode>),
    Literal(u64),
    GreaterThan(Box<AstNode>, Box<AstNode>),
    LessThan(Box<AstNode>, Box<AstNode>),
    EqualTo(Box<AstNode>, Box<AstNode>),
}

impl AstNode {
    pub fn evaluate_input(input: &str) -> Result<u64, Error> {
        let mut builder = AstBuilder::new(input);
        let ast = builder.build()?;
        Ok(ast.evaluate())
    }

    pub fn evaluate(self) -> u64 {
        match self {
            AstNode::Sum(nodes) => {
                let mut sum = 0;
                for node in nodes {
                    sum += node.evaluate();
                }
                sum
            }
            AstNode::Product(nodes) => {
                let mut product = 1;
                for node in nodes {
                    product *= node.evaluate();
                }
                product
            }
            AstNode::Minimum(nodes) => {
                let mut minimum = u64::MAX;
                for node in nodes {
                    minimum = std::cmp::min(minimum, node.evaluate());
                }
                minimum
            }
            AstNode::Maximum(nodes) => {
                let mut maximum = 0;
                for node in nodes {
                    maximum = std::cmp::max(maximum, node.evaluate());
                }
                maximum
            }
            AstNode::Literal(v) => v,
            AstNode::GreaterThan(n1, n2) => {
                let v1 = n1.evaluate();
                let v2 = n2.evaluate();
                if v1 > v2 {
                    1
                } else {
                    0
                }
            }
            AstNode::LessThan(n1, n2) => {
                let v1 = n1.evaluate();
                let v2 = n2.evaluate();
                if v1 < v2 {
                    1
                } else {
                    0
                }
            }
            AstNode::EqualTo(n1, n2) => {
                let v1 = n1.evaluate();
                let v2 = n2.evaluate();
                if v1 == v2 {
                    1
                } else {
                    0
                }
            }
        }
    }
}

pub struct AstBuilder<'a> {
    packet_parser: PacketParser<'a>,
}

impl<'a> AstBuilder<'a> {
    pub fn new(input: &'a str) -> Self {
        let packet_parser = PacketParser::new(input);
        Self { packet_parser }
    }

    pub fn build(&mut self) -> Result<AstNode, Error> {
        self.build_bits().map(|(_bits, node)| node)
    }

    pub fn build_bits(&mut self) -> Result<(u16, AstNode), Error> {
        let packet = self.packet_parser.next()?.ok_or(Error::ExpectedPacket)?;
        let bits = packet.bits();
        let (sub_bits, ast) = match packet.type_id.into_u8() {
            0 => {
                let (bits, vec) = self.build_vec(packet.payload)?;
                (bits, AstNode::Sum(vec))
            }
            1 => {
                let (bits, vec) = self.build_vec(packet.payload)?;
                (bits, AstNode::Product(vec))
            }
            2 => {
                let (bits, vec) = self.build_vec(packet.payload)?;
                (bits, AstNode::Minimum(vec))
            }
            3 => {
                let (bits, vec) = self.build_vec(packet.payload)?;
                (bits, AstNode::Maximum(vec))
            }
            4 => (
                0,
                AstNode::Literal(packet.payload.as_literal().ok_or(Error::ExpectedLiteral)?),
            ),
            5 => {
                let (b1, ast1) = self.build_bits()?;
                let (b2, ast2) = self.build_bits()?;
                let bits = b1 + b2;
                let node = AstNode::GreaterThan(Box::new(ast1), Box::new(ast2));
                (bits, node)
            }
            6 => {
                let (b1, ast1) = self.build_bits()?;
                let (b2, ast2) = self.build_bits()?;
                let bits = b1 + b2;
                let node = AstNode::LessThan(Box::new(ast1), Box::new(ast2));
                (bits, node)
            }
            7 => {
                let (b1, ast1) = self.build_bits()?;
                let (b2, ast2) = self.build_bits()?;
                let bits = b1 + b2;
                let node = AstNode::EqualTo(Box::new(ast1), Box::new(ast2));
                (bits, node)
            }
            _ => return Err(Error::InvalidTypeId(packet.type_id)),
        };
        Ok((bits + sub_bits, ast))
    }

    pub fn build_vec(&mut self, payload: Payload) -> Result<(u16, Vec<AstNode>), Error> {
        match payload {
            Payload::OperatorBitLength(bits) => self.build_vec_bits(bits),
            Payload::OperatorPacketLength(packets) => self.build_vec_packets(packets),
            Payload::Literal { .. } => Err(Error::UnexpectedLiteral),
        }
    }

    pub fn build_vec_bits(&mut self, bits: u16) -> Result<(u16, Vec<AstNode>), Error> {
        let mut running_total_of_bits = 0;
        let mut vec = Vec::new();
        while running_total_of_bits < bits {
            let (sub_bits, node) = self.build_bits()?;
            running_total_of_bits += sub_bits;
            vec.push(node)
        }
        if running_total_of_bits != bits {
            return Err(Error::MisalignedBits);
        }

        Ok((running_total_of_bits, vec))
    }

    pub fn build_vec_packets(&mut self, packets: u16) -> Result<(u16, Vec<AstNode>), Error> {
        let mut running_total_of_bits = 0;
        let mut vec = Vec::with_capacity(packets as usize);
        for _ in 0..packets {
            let (sub_bits, node) = self.build_bits()?;
            running_total_of_bits += sub_bits;
            vec.push(node);
        }
        Ok((running_total_of_bits, vec))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    PacketParser(#[from] crate::packet::Error),

    #[error("Expected packet but did not receive one")]
    ExpectedPacket,

    #[error("Invalid type ID: {0:?}")]
    InvalidTypeId(TypeId),

    #[error("Not implemented")]
    NotImplemented,

    #[error("Expected literal")]
    ExpectedLiteral,

    #[error("Unexpected literal")]
    UnexpectedLiteral,

    #[error("Misaligned bits")]
    MisalignedBits,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate() {
        let ast = AstNode::Literal(17);
        assert_eq!(ast.evaluate(), 17);

        let ast = AstNode::Sum(vec![
            AstNode::Literal(1),
            AstNode::Literal(2),
            AstNode::Literal(3),
            AstNode::Literal(4),
        ]);
        assert_eq!(ast.evaluate(), 10);

        let ast = AstNode::Product(vec![
            AstNode::Literal(1),
            AstNode::Literal(2),
            AstNode::Literal(3),
            AstNode::Literal(4),
        ]);
        assert_eq!(ast.evaluate(), 24);

        let ast = AstNode::Minimum(vec![
            AstNode::Literal(1),
            AstNode::Literal(2),
            AstNode::Literal(3),
            AstNode::Literal(4),
        ]);
        assert_eq!(ast.evaluate(), 1);

        let ast = AstNode::Maximum(vec![
            AstNode::Literal(1),
            AstNode::Literal(2),
            AstNode::Literal(3),
            AstNode::Literal(4),
        ]);
        assert_eq!(ast.evaluate(), 4);

        let ast =
            AstNode::GreaterThan(Box::new(AstNode::Literal(2)), Box::new(AstNode::Literal(3)));
        assert_eq!(ast.evaluate(), 0);

        let ast =
            AstNode::GreaterThan(Box::new(AstNode::Literal(3)), Box::new(AstNode::Literal(2)));
        assert_eq!(ast.evaluate(), 1);

        let ast = AstNode::LessThan(Box::new(AstNode::Literal(2)), Box::new(AstNode::Literal(3)));
        assert_eq!(ast.evaluate(), 1);

        let ast = AstNode::LessThan(Box::new(AstNode::Literal(3)), Box::new(AstNode::Literal(2)));
        assert_eq!(ast.evaluate(), 0);

        let ast = AstNode::EqualTo(Box::new(AstNode::Literal(2)), Box::new(AstNode::Literal(3)));
        assert_eq!(ast.evaluate(), 0);

        let ast = AstNode::EqualTo(Box::new(AstNode::Literal(3)), Box::new(AstNode::Literal(3)));
        assert_eq!(ast.evaluate(), 1);
    }

    #[test]
    pub fn test_builder() {
        let mut ast_builder = AstBuilder::new("C200B40A82");
        let ast = ast_builder.build().unwrap();
        assert_eq!(
            ast,
            AstNode::Sum(vec![AstNode::Literal(1), AstNode::Literal(2)])
        );
        assert_eq!(ast.evaluate(), 3);

        let mut ast_builder = AstBuilder::new("04005AC33890");
        let ast = ast_builder.build().unwrap();
        assert_eq!(
            ast,
            AstNode::Product(vec![AstNode::Literal(6), AstNode::Literal(9)])
        );
        assert_eq!(ast.evaluate(), 54);

        let mut ast_builder = AstBuilder::new("880086C3E88112");
        let ast = ast_builder.build().unwrap();
        assert_eq!(
            ast,
            AstNode::Minimum(vec![
                AstNode::Literal(7),
                AstNode::Literal(8),
                AstNode::Literal(9)
            ])
        );
        assert_eq!(ast.evaluate(), 7);

        assert!(matches!(AstNode::evaluate_input("D8005AC2A8F0"), Ok(1)));
        assert!(matches!(AstNode::evaluate_input("F600BC2D8F"), Ok(0)));
        assert!(matches!(AstNode::evaluate_input("9C005AC2F8F0"), Ok(0)));
        assert!(matches!(
            AstNode::evaluate_input("9C0141080250320F1802104A08"),
            Ok(1)
        ));
    }
}
