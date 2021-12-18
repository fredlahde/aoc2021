use bitvec::prelude::Msb0;
use bitvec::prelude::*;
use bitvec::slice::BitSlice;
use std::ops::{BitXor, Index, Range, Shl};

fn main() {
    use std::io::Read;
    let mut fd = std::fs::File::open("input").unwrap();
    let mut contents = String::new();
    fd.read_to_string(&mut contents).unwrap();
    let contents = contents.replace('\n', "");
    let bits = BitStream::from(contents.as_str());
    let packets = parse_bit_stream(bits).unwrap();

    let part1 = packets.iter().map(|p| p.version).fold(0u128, |acc, x| {
        acc + x as u128
    });
    println!("{}", part1);
}

const LITERAL_PACKET_TYP: u8 = 4;

const LENGTH_TYP_LENGTH_IN_BYTES: u8 = 0;
const LENGTH_TYP_NUM_SUB_PACKETS: u8 = 1;

pub struct BitStream {
    backing: BitVec<Msb0, u8>,
    idx: usize,
}

impl From<&str> for BitStream {
    fn from(s: &str) -> Self {
        assert!(s.len() % 2 == 0);
        let mut backing = Vec::new();
        let s_len = s.len();
        let mut chars = s.chars();
        for _n in 0..(s_len / 2) {
            let a: u8 = chars
                .next()
                .unwrap()
                .to_digit(16)
                .unwrap()
                .try_into()
                .unwrap();
            let b: u8 = chars
                .next()
                .unwrap()
                .to_digit(16)
                .unwrap()
                .try_into()
                .unwrap();
            let x = a << 4 | b;
            backing.push(x);
        }
        assert!(backing.len() == s.len() / 2);
        Self {
            backing: backing.view_bits::<Msb0>().to_owned(),
            idx: 0,
        }
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
    ) -> Result<T, ConversionError> {
        if self.idx + bits > self.backing.len() {
            return Err(ConversionError::OOB);
        }
        let ret = convert(&self[self.idx..self.idx + bits]);
        self.idx += bits;
        ret
    }
}

#[derive(Debug)]
pub enum ConversionError {
    OOB,
    Overflow,
}

pub fn convert<T: From<bool> + BitXor<Output = T> + Shl<Output = T>>(
    bits: &BitSlice<Msb0, u8>,
) -> Result<T, ConversionError> {
    let have_space = std::mem::size_of::<T>() * 8;
    if bits.len() > have_space {
        return Err(ConversionError::Overflow);
    }

    let ret = bits.iter().fold(T::from(false), |result, bit| {
        (result << T::from(true)) ^ T::from(*bit)
    });
    Ok(ret)
}

#[derive(Debug, PartialEq)]
pub struct Packet {
    version: u8,
    typ: u8,
    val: u128,
}

pub fn parse_bit_stream(mut stream: BitStream) -> Result<Vec<Packet>, ConversionError> {
    let mut ret = Vec::new();
    parse_bits_intern(&mut stream, &mut ret)?;
    Ok(ret)
}

fn parse_bits_intern(
    stream: &mut BitStream,
    acc: &mut Vec<Packet>,
) -> Result<usize, ConversionError> {
    let mut read_all: usize = 0;
    let version: u8 = stream.consume(3)?;
    let typ: u8 = stream.consume(3)?;
    read_all += 6;

    match typ {
        LITERAL_PACKET_TYP => {
            let mut cont: u8 = stream.consume(1)?;
            read_all += 1;
            let mut acc_local = vec![stream.consume::<u8>(4)?];
            read_all += 4;

            while cont == 1 {
                cont = stream.consume(1)?;
                acc_local.push(stream.consume::<u8>(4)?);
                read_all += 5;
            }

            let mut val: u128 = 0;
            for xx in &acc_local {
                val = (val << 4) | *xx as u128;
            }
            acc.push(Packet { version, typ, val });
        }
        _ => {
            let len_typ: u8 = stream.consume(1)?;
            read_all += 1;
            match len_typ {
                LENGTH_TYP_LENGTH_IN_BYTES => {
                    let sub_packet_len: u16 = stream.consume(15)?;
                    read_all += 15;

                    acc.push(Packet {
                        version,
                        typ,
                        val: 0,
                    });
                    let mut read: usize = 0;
                    while read < sub_packet_len.into() {
                        read += parse_bits_intern(stream, acc)?;
                    }
                    read_all += read;
                }
                LENGTH_TYP_NUM_SUB_PACKETS => {
                    let num_packets: u16 = stream.consume(11)?;
                    read_all += 11;
                    let mut packets_found = 0;
                    acc.push(Packet {
                        version,
                        typ,
                        val: 0,
                    });
                    while packets_found < num_packets {
                        read_all += parse_bits_intern(stream, acc)?;
                        packets_found += 1;
                    }
                }
                _ => todo!(),
            };
        }
    };

    Ok(read_all)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_packet_stream() {
        let s = "D2FE28";
        let bin: BitStream = s.into();
        assert_eq!(vec![0xd2, 0xfe, 0x28], bin.backing.as_raw_slice());

        let s = "EE00D40C823060";
        let bin: BitStream = s.into();
        assert_eq!(
            vec![0xEE, 0x00, 0xD4, 0x0C, 0x82, 0x30, 0x60],
            bin.backing.as_raw_slice()
        );
        assert_eq!(238, convert(&bin[0..8]).unwrap());
        assert_eq!(212, convert(&bin[16..24]).unwrap());

        let s = "FF";
        let bin: BitStream = s.into();
        assert_eq!(255, convert(&bin[0..8]).unwrap());
        assert_eq!(15, convert(&bin[0..4]).unwrap());

        let s = "FFFF";
        let mut bin: BitStream = s.into();
        assert_eq!(15, bin.consume(4).unwrap());
        assert_eq!(15, bin.consume(4).unwrap());
        assert_eq!(255, bin.consume(8).unwrap());

        let s = "D2FE28";
        let mut bin: BitStream = s.into();
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
        let bin: BitStream = s.into();
        let packets = parse_bit_stream(bin).unwrap();
        assert_eq!(1, packets.len());
        assert_eq!(
            Packet {
                version: 6,
                typ: 4,
                val: 2021
            },
            packets[0]
        );

        let s = "38006F45291200";
        let bin: BitStream = s.into();
        let packets = parse_bit_stream(bin).unwrap();
        assert_eq!(3, packets.len());
        assert_eq!(
            Packet {
                version: 1,
                typ: 6,
                val: 0
            },
            packets[0]
        );
        assert_eq!(
            Packet {
                version: 6,
                typ: 4,
                val: 10
            },
            packets[1]
        );
        assert_eq!(
            Packet {
                version: 2,
                typ: 4,
                val: 20
            },
            packets[2]
        );

        let s = "EE00D40C823060";
        let bin: BitStream = s.into();
        let packets = parse_bit_stream(bin).unwrap();
        assert_eq!(4, packets.len());
        assert_eq!(
            Packet {
                version: 7,
                typ: 3,
                val: 0
            },
            packets[0]
        );
        assert_eq!(
            Packet {
                version: 2,
                typ: 4,
                val: 1
            },
            packets[1]
        );
        assert_eq!(
            Packet {
                version: 4,
                typ: 4,
                val: 2
            },
            packets[2]
        );
        assert_eq!(
            Packet {
                version: 1,
                typ: 4,
                val: 3
            },
            packets[3]
        );
    }
}
