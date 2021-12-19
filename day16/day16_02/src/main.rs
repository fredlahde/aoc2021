//! A `BITS` packet parser
#![deny(missing_docs)]
#![deny(clippy::missing_docs_in_private_items)]

use bitvec::prelude::Msb0;
use bitvec::prelude::*;
use bitvec::slice::BitSlice;
use std::num::TryFromIntError;
use std::ops::{BitXor, Index, Range, Shl};

fn main() {
    use std::io::Read;
    let mut fd = std::fs::File::open("input").unwrap();
    let mut contents = String::new();
    fd.read_to_string(&mut contents).unwrap();
    let contents = contents.replace('\n', "");
    let bits = BitStream::try_from(contents.as_str()).unwrap();
    let packet = parse_bit_stream(bits).unwrap();
    let res = apply_ops(&packet);
    assert_eq!(831996589851, res);
    println!("{}", res);
}

/// The `type` field of a `literal` packet
const LITERAL_PACKET_TYP: u8 = 4;

/// The `type` field of a `sum` packet
const SUM_PACKET_TYP: u8 = 0;

/// The `type` field of a `product` packet
const PRODUCT_PACKET_TYPE: u8 = 1;

/// The `type` field of a `minimum` packet
const MINIMUM_PACKET_TYPE: u8 = 2;

/// The `type` field of a `maximum` packet
const MAXIMUM_PACKET_TYPE: u8 = 3;

/// The `type` field of a `greater than` packet
const GREATER_THAN_PACKET_TYPE: u8 = 5;

/// The `type` field of a `less than` packet
const LESS_THAN_PACKET_TYPE: u8 = 6;

/// The `type` field of a `equals` packet
const EQ_PACKET_TYPE: u8 = 7;

/// The `length type` bit for a packet describing it's sub packets in length of bits
const LENGTH_TYP_LENGTH_IN_BITS: u8 = 0;

/// The `length type` bit for a packet describing it's sub packets in number of packets
const LENGTH_TYP_NUM_SUB_PACKETS: u8 = 1;

/// The length in bits for the `version` field
const BITS_VERSION: usize = 3;

/// The length in bits for the `typ` field
const BITS_TYP: usize = 3;

/// The length in bits for the `len typ` field
const BITS_LEN_TYP: usize = 1;

/// The length in bits for the `continue bit` field
const BITS_CONT: usize = 1;

/// The length in bits for the `literal value` field
const BITS_LITERAL_VAL: usize = 4;

/// The length in bits for the `sub packet length` field
const BITS_SUB_PACKET_LENGTH: usize = 15;

/// The length in bits for the `sub packet count` field
const BITS_SUB_PACKET_COUNT: usize = 11;

/// A stream of bits, backed by a vector of bytes
pub struct BitStream {
    /// The backing vector of bytes
    backing: BitVec<Msb0, u8>,

    /// The current index into `backing`
    /// Get's increased by calling `consume`
    idx: usize,
}

/// Errors that can occur while parsing the packets
#[derive(Debug, Clone)]
pub enum ParsingError {
    /// Failing to read more bits then avaiable
    OOB,

    /// Failing to parse bits into a too-small integer type
    Overflow,

    /// Failing to convert a char that is not hex into a integer
    InvalidHexDigit(char),

    /// Failing to safely cast a integer type
    FromIntError(TryFromIntError),

    /// The input hex string ended earlier than expected
    EOF,

    /// The packet stream is empty
    EmptyPacketStream,

    /// The input has not a valid length
    InvalidInputLen,

    /// Parsing encountered an invalid operand
    InvalidOperand(u8),

    /// Parsing encountered an invalid length typ
    InvalidLengthTyp(u8),
}

impl From<TryFromIntError> for ParsingError {
    fn from(e: TryFromIntError) -> Self {
        Self::FromIntError(e)
    }
}

/// Tries to convert two hex chars into a byte
fn byte_from_hex_chars(chars: (char, char)) -> Result<u8, ParsingError> {
    let mut val: u8 = 0;
    for c in [chars.0, chars.1] {
        let byte: u8 = c
            .to_digit(16)
            .ok_or(ParsingError::InvalidHexDigit(c))?
            .try_into()?;
        val = (val << 4) | byte;
    }
    Ok(val)
}

impl TryFrom<&str> for BitStream {
    type Error = ParsingError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s.len() % 2 != 0 {
            return Err(ParsingError::InvalidInputLen);
        }
        s.chars()
            .step_by(2)
            .zip(s.chars().skip(1).step_by(2))
            .map(byte_from_hex_chars)
            .collect::<Result<Vec<_>, _>>()
            .map(|backing| Self {
                backing: backing.view_bits::<Msb0>().to_owned(),
                idx: 0,
            })
    }
}

impl Index<Range<usize>> for BitStream {
    type Output = BitSlice<Msb0, u8>;

    fn index(&self, range: Range<usize>) -> &Self::Output {
        &self.backing[range]
    }
}

impl BitStream {
    /// Consume `bits` number of bits from this stream and try to convert it into an integer
    fn consume<T: From<bool> + BitXor<Output = T> + Shl<Output = T>>(
        &mut self,
        bits: usize,
    ) -> Result<T, ParsingError> {
        if self.idx + bits > self.backing.len() {
            return Err(ParsingError::OOB);
        }
        let ret = convert(&self[self.idx..self.idx + bits]);
        self.idx += bits;
        ret
    }
}

/// Try to convert a set of bits into a integer
fn convert<T: From<bool> + BitXor<Output = T> + Shl<Output = T>>(
    bits: &BitSlice<Msb0, u8>,
) -> Result<T, ParsingError> {
    let have_space = std::mem::size_of::<T>() * 8;
    if bits.len() > have_space {
        return Err(ParsingError::Overflow);
    }

    let ret = bits.iter().fold(T::from(false), |result, bit| {
        (result << T::from(true)) ^ T::from(*bit)
    });
    Ok(ret)
}

/// A packet in the `BITS` system
#[derive(Debug, PartialEq)]
pub struct Packet {
    /// The version header of this packet
    version: u8,

    /// The pay load of this packet.
    /// Either a literal value of an op on a list of sub packets
    payload: Payload,
}

/// The payload of a `BITS` packet.
/// Can either be a literal value or an operation on a set of packets
#[derive(Debug, PartialEq)]
pub enum Payload {
    /// Payload that simpy hols a literal value
    Literal(u128),

    /// A operation describing how to alter the child packets
    Op(Operand, Vec<Packet>),
}

/// A operation that has to be applied on a set of packets
#[derive(Debug, PartialEq)]
pub enum Operand {
    /// Calculate the sum of the set of packets
    Sum,

    /// Calculate the product of the set of packets
    Product,

    /// Calculate the minimum of the set of packets
    Minimum,

    /// Calculate the maximum of the set of packets
    Maximum,

    /// Calculate whether the first packet is greater than the second packet
    /// This operation always operates on exactly two packets
    GreaterThan,

    /// Calculate whether the first packet is less than the second packet
    /// This operation always operates on exactly two packets
    LessThan,

    /// Calculate whether the first packet is equal to the second packet
    /// This operation always operates on exactly two packets
    Equals,
}

impl TryFrom<u8> for Operand {
    type Error = ParsingError;

    fn try_from(x: u8) -> Result<Self, Self::Error> {
        Ok(match x {
            SUM_PACKET_TYP => Operand::Sum,
            PRODUCT_PACKET_TYPE => Operand::Product,
            MINIMUM_PACKET_TYPE => Operand::Minimum,
            MAXIMUM_PACKET_TYPE => Operand::Maximum,
            GREATER_THAN_PACKET_TYPE => Operand::GreaterThan,
            LESS_THAN_PACKET_TYPE => Operand::LessThan,
            EQ_PACKET_TYPE => Operand::Equals,
            _ => return Err(ParsingError::InvalidOperand(x)),
        })
    }
}

impl Packet {
    /// Construct a new `literal` packet
    fn new_literal_packet(version: u8, val: u128) -> Self {
        Self {
            version,
            payload: Payload::Literal(val),
        }
    }

    /// Construct a new `operation` packet with child packets
    fn new_op_packet(version: u8, op: Operand, childs: Vec<Packet>) -> Self {
        Self {
            version,
            payload: Payload::Op(op, childs),
        }
    }
}

/// Consumes `n` bits from the stream `bin` and increases `acc` by `n`
macro_rules! cat {
    ($acc: ident, $bin: ident, $n: ident) => {{
        let x = $bin.consume($n)?;
        $acc += $n;
        x
    }};
}

/// Parse a stream of bits into a `BITS` packet tree
pub fn parse_bit_stream(mut stream: BitStream) -> Result<Packet, ParsingError> {
    let mut ret = Vec::new();
    parse_bits_intern(&mut stream, &mut ret)?;
    ret.pop().ok_or(ParsingError::EmptyPacketStream)
}

/// Recursivley parse a stream of bits into a `BITS` packet tree
fn parse_bits_intern(stream: &mut BitStream, acc: &mut Vec<Packet>) -> Result<usize, ParsingError> {
    let mut read_all: usize = 0;
    let version: u8 = cat!(read_all, stream, BITS_VERSION);
    let typ: u8 = cat!(read_all, stream, BITS_TYP);

    match typ {
        LITERAL_PACKET_TYP => {
            let mut cont: u8 = cat!(read_all, stream, BITS_CONT);
            let val: u8 = cat!(read_all, stream, BITS_LITERAL_VAL);
            let mut acc_local = vec![val];

            while cont == 1 {
                cont = cat!(read_all, stream, BITS_CONT);
                acc_local.push(cat!(read_all, stream, BITS_LITERAL_VAL));
            }

            let val = acc_local
                .into_iter()
                .fold(0u128, |acc, x| (acc << 4) | x as u128);
            acc.push(Packet::new_literal_packet(version, val));
        }
        _ => {
            let len_typ: u8 = cat!(read_all, stream, BITS_LEN_TYP);
            let mut local_acc = Vec::new();
            match len_typ {
                LENGTH_TYP_LENGTH_IN_BITS => {
                    let sub_packet_len: u16 = cat!(read_all, stream, BITS_SUB_PACKET_LENGTH);

                    let mut read: usize = 0;
                    while read < sub_packet_len.into() {
                        read += parse_bits_intern(stream, &mut local_acc)?;
                    }
                    read_all += read;
                }
                LENGTH_TYP_NUM_SUB_PACKETS => {
                    let num_packets: u16 = cat!(read_all, stream, BITS_SUB_PACKET_COUNT);
                    let mut packets_found = 0;
                    while packets_found < num_packets {
                        read_all += parse_bits_intern(stream, &mut local_acc)?;
                        packets_found += 1;
                    }
                }
                _ => return Err(ParsingError::InvalidLengthTyp(len_typ)),
            };

            let op = Operand::try_from(typ)?;
            if matches!(
                op,
                Operand::GreaterThan | Operand::LessThan | Operand::Equals
            ) && local_acc.len() != 2
            {
                return Err(ParsingError::InvalidInputLen);
            }
            acc.push(Packet::new_op_packet(version, op, local_acc));
        }
    };

    Ok(read_all)
}

/// Apply the operations of this packet and it's sub-packets recursively
pub fn apply_ops(packet: &Packet) -> u128 {
    match &packet.payload {
        Payload::Literal(x) => *x,
        Payload::Op(op, childs) => match op {
            Operand::Sum => childs.iter().map(apply_ops).sum(),
            Operand::Product => childs.iter().map(apply_ops).product(),
            Operand::Minimum => childs.iter().map(apply_ops).min().unwrap_or(0),
            Operand::Maximum => childs.iter().map(apply_ops).max().unwrap_or(0),
            Operand::LessThan => {
                let a = apply_ops(&childs[0]);
                let b = apply_ops(&childs[1]);
                (a < b) as u128
            }
            Operand::GreaterThan => {
                let a = apply_ops(&childs[0]);
                let b = apply_ops(&childs[1]);
                (a > b) as u128
            }
            Operand::Equals => {
                let a = apply_ops(&childs[0]);
                let b = apply_ops(&childs[1]);
                (a == b) as u128
            }
        },
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_packet_stream() {
        let s = "D2FE28";
        let bin: BitStream = s.try_into().unwrap();
        assert_eq!(vec![0xd2, 0xfe, 0x28], bin.backing.as_raw_slice());

        let s = "EE00D40C823060";
        let bin: BitStream = s.try_into().unwrap();
        assert_eq!(
            vec![0xEE, 0x00, 0xD4, 0x0C, 0x82, 0x30, 0x60],
            bin.backing.as_raw_slice()
        );
        assert_eq!(238, convert(&bin[0..8]).unwrap());
        assert_eq!(212, convert(&bin[16..24]).unwrap());

        let s = "FF";
        let bin: BitStream = s.try_into().unwrap();
        assert_eq!(255, convert(&bin[0..8]).unwrap());
        assert_eq!(15, convert(&bin[0..4]).unwrap());

        let s = "FFFF";
        let mut bin: BitStream = s.try_into().unwrap();
        assert_eq!(15, bin.consume(4).unwrap());
        assert_eq!(15, bin.consume(4).unwrap());
        assert_eq!(255, bin.consume(8).unwrap());

        let s = "D2FE28";
        let mut bin: BitStream = s.try_into().unwrap();
        assert_eq!(6, bin.consume(3).unwrap());
        assert_eq!(4, bin.consume(3).unwrap());
        assert_eq!(1, bin.consume(1).unwrap());
        let a: u8 = bin.consume(4).unwrap();
        assert_eq!(1, bin.consume(1).unwrap());
        let b: u8 = bin.consume(4).unwrap();
        assert_eq!(0, bin.consume(1).unwrap());
        let c: u8 = bin.consume(4).unwrap();
        let x = (a as u64) << 4 | b as u64;
        let x = x << 4 | c as u64;
        assert_eq!(2021, x);
        assert_eq!(0, bin.consume(3).unwrap());
    }

    #[test]
    fn test_parse_packets() {
        let s = "D2FE28";
        let bin: BitStream = s.try_into().unwrap();
        let packet = parse_bit_stream(bin).unwrap();
        assert_eq!(
            Packet {
                version: 6,
                payload: Payload::Literal(2021)
            },
            packet
        );

        let s = "38006F45291200";
        let bin: BitStream = s.try_into().unwrap();
        let packet = parse_bit_stream(bin).unwrap();
        assert_eq!(
            Packet {
                version: 1,
                payload: Payload::Op(
                    Operand::LessThan,
                    vec![
                        Packet {
                            version: 6,
                            payload: Payload::Literal(10)
                        },
                        Packet {
                            version: 2,
                            payload: Payload::Literal(20)
                        }
                    ]
                )
            },
            packet
        );

        let s = "EE00D40C823060";
        let bin: BitStream = s.try_into().unwrap();
        let packet = parse_bit_stream(bin).unwrap();
        assert_eq!(
            Packet {
                version: 7,
                payload: Payload::Op(
                    Operand::Maximum,
                    vec![
                        Packet {
                            version: 2,
                            payload: Payload::Literal(1)
                        },
                        Packet {
                            version: 4,
                            payload: Payload::Literal(2)
                        },
                        Packet {
                            version: 1,
                            payload: Payload::Literal(3)
                        }
                    ]
                )
            },
            packet
        );
    }

    #[test]
    fn test_ops() {
        let s = "C200B40A82";
        let bin: BitStream = s.try_into().unwrap();
        let packet = parse_bit_stream(bin).unwrap();
        assert_eq!(
            Packet {
                version: 6,
                payload: Payload::Op(
                    Operand::Sum,
                    vec![
                        Packet {
                            version: 6,
                            payload: Payload::Literal(1)
                        },
                        Packet {
                            version: 2,
                            payload: Payload::Literal(2)
                        },
                    ]
                )
            },
            packet
        );
        let res = apply_ops(&packet);
        assert_eq!(3, res);

        let s = "04005AC33890";
        let bin: BitStream = s.try_into().unwrap();
        let packet = parse_bit_stream(bin).unwrap();
        let res = apply_ops(&packet);
        assert_eq!(54, res);

        let s = "880086C3E88112";
        let bin: BitStream = s.try_into().unwrap();
        let packet = parse_bit_stream(bin).unwrap();
        let res = apply_ops(&packet);
        assert_eq!(7, res);

        let s = "CE00C43D881120";
        let bin: BitStream = s.try_into().unwrap();
        let packet = parse_bit_stream(bin).unwrap();
        let res = apply_ops(&packet);
        assert_eq!(9, res);

        let s = "D8005AC2A8F0";
        let bin: BitStream = s.try_into().unwrap();
        let packet = parse_bit_stream(bin).unwrap();
        let res = apply_ops(&packet);
        assert_eq!(1, res);

        let s = "F600BC2D8F";
        let bin: BitStream = s.try_into().unwrap();
        let packet = parse_bit_stream(bin).unwrap();
        let res = apply_ops(&packet);
        assert_eq!(0, res);

        let s = "9C005AC2F8F0";
        let bin: BitStream = s.try_into().unwrap();
        let packet = parse_bit_stream(bin).unwrap();
        let res = apply_ops(&packet);
        assert_eq!(0, res);

        let s = "9C0141080250320F1802104A08";
        let bin: BitStream = s.try_into().unwrap();
        let packet = parse_bit_stream(bin).unwrap();
        let res = apply_ops(&packet);
        assert_eq!(1, res);
    }
}
