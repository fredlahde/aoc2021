use bitvec::prelude::Msb0;
use bitvec::prelude::*;
use bitvec::slice::BitSlice;
use std::convert::Infallible;
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

const LITERAL_PACKET_TYP: u8 = 4;
const SUM_PACKET_TYP: u8 = 0;
const PRODUCT_PACKET_TYPE: u8 = 1;
const MINIMUM_PACKET_TYPE: u8 = 2;
const MAXIMUM_PACKET_TYPE: u8 = 3;
const GREATER_THAN_PACKET_TYPE: u8 = 5;
const LESS_THAN_PACKET_TYPE: u8 = 6;
const EQ_PACKET_TYPE: u8 = 7;

const LENGTH_TYP_LENGTH_IN_BYTES: u8 = 0;
const LENGTH_TYP_NUM_SUB_PACKETS: u8 = 1;

pub struct BitStream {
    backing: BitVec<Msb0, u8>,
    idx: usize,
}

#[derive(Debug, Clone)]
pub enum ParsingError {
    OOB,
    Overflow,
    InvalidHexDigit(char),
    FromIntError(TryFromIntError),
    ByteOverflow(Infallible),
    EOF,
    EmptyPacketStream,
    InvalidInputLen,
}

impl From<Infallible> for ParsingError {
    fn from(e: Infallible) -> Self {
        Self::ByteOverflow(e)
    }
}

impl From<TryFromIntError> for ParsingError {
    fn from(e: TryFromIntError) -> Self {
        Self::FromIntError(e)
    }
}

fn byte_from_hex_char(c: char) -> Result<u8, ParsingError> {
    Ok(c.to_digit(16)
        .ok_or(ParsingError::InvalidHexDigit(c))?
        .try_into()?)
}

impl TryFrom<&str> for BitStream {
    type Error = ParsingError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s.len() % 2 != 0 {
            return Err(ParsingError::InvalidInputLen);
        }
        let s_len = s.len();
        let mut chars = s.chars();
        let backing: Vec<Result<u8, ParsingError>> = (0..(s_len / 2))
            .map(|_| {
                let a = chars
                    .next()
                    .ok_or(ParsingError::EOF)
                    .and_then(byte_from_hex_char);
                let b = chars
                    .next()
                    .ok_or(ParsingError::EOF)
                    .and_then(byte_from_hex_char);
                if a.is_err() {
                    return a;
                }
                if b.is_err() {
                    return b;
                }
                Ok(a.unwrap() << 4 | b.unwrap())
            })
            .collect();
        if backing.len() != s.len() / 2 {
            return Err(ParsingError::InvalidInputLen);
        }

        let maybe_err = backing.iter().find(|r| r.is_err());
        if let Some(Err(e)) = maybe_err {
            return Err(e.clone());
        }

        let backing: Vec<u8> = backing.into_iter().map(|r| r.unwrap()).collect();

        Ok(Self {
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

#[derive(Debug, PartialEq)]
pub struct Packet {
    version: u8,
    payload: Payload,
}

#[derive(Debug, PartialEq)]
pub enum Payload {
    Literal(u128),
    Op(Operand, Vec<Packet>),
}

#[derive(Debug, PartialEq)]
pub enum Operand {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    Equals,
}

impl From<u8> for Operand {
    fn from(x: u8) -> Self {
        match x {
            SUM_PACKET_TYP => Operand::Sum,
            PRODUCT_PACKET_TYPE => Operand::Product,
            MINIMUM_PACKET_TYPE => Operand::Minimum,
            MAXIMUM_PACKET_TYPE => Operand::Maximum,
            GREATER_THAN_PACKET_TYPE => Operand::GreaterThan,
            LESS_THAN_PACKET_TYPE => Operand::LessThan,
            EQ_PACKET_TYPE => Operand::Equals,
            _ => todo!(),
        }
    }
}

impl Packet {
    fn new_literal_packet(version: u8, val: u128) -> Self {
        Self {
            version,
            payload: Payload::Literal(val),
        }
    }

    fn new_op_packet(version: u8, op: Operand, childs: Vec<Packet>) -> Self {
        Self {
            version,
            payload: Payload::Op(op, childs),
        }
    }
}

macro_rules! cat {
    ($acc: ident, $bin: ident, $n: literal) => {{
        let x = $bin.consume($n)?;
        $acc += $n;
        x
    }};
}

pub fn parse_bit_stream(mut stream: BitStream) -> Result<Packet, ParsingError> {
    let mut ret = Vec::new();
    parse_bits_intern(&mut stream, &mut ret)?;
    ret.pop().ok_or(ParsingError::EmptyPacketStream)
}

fn parse_bits_intern(stream: &mut BitStream, acc: &mut Vec<Packet>) -> Result<usize, ParsingError> {
    let mut read_all: usize = 0;
    let version: u8 = cat!(read_all, stream, 3);
    let typ: u8 = cat!(read_all, stream, 3);

    match typ {
        LITERAL_PACKET_TYP => {
            let mut cont: u8 = cat!(read_all, stream, 1);
            let val: u8 = cat!(read_all, stream, 4);
            let mut acc_local = vec![val];

            while cont == 1 {
                cont = cat!(read_all, stream, 1);
                acc_local.push(cat!(read_all, stream, 4));
            }

            let val = acc_local
                .into_iter()
                .fold(0u128, |acc, x| (acc << 4) | x as u128);
            acc.push(Packet::new_literal_packet(version, val));
        }
        _ => {
            let len_typ: u8 = cat!(read_all, stream, 1);
            let mut local_acc = Vec::new();
            match len_typ {
                LENGTH_TYP_LENGTH_IN_BYTES => {
                    let sub_packet_len: u16 = cat!(read_all, stream, 15);

                    let mut read: usize = 0;
                    while read < sub_packet_len.into() {
                        read += parse_bits_intern(stream, &mut local_acc)?;
                    }
                    read_all += read;
                }
                LENGTH_TYP_NUM_SUB_PACKETS => {
                    let num_packets: u16 = cat!(read_all, stream, 11);
                    let mut packets_found = 0;
                    while packets_found < num_packets {
                        read_all += parse_bits_intern(stream, &mut local_acc)?;
                        packets_found += 1;
                    }
                }
                _ => todo!(),
            };

            let op = Operand::from(typ);
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
