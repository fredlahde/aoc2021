use bitvec::prelude::Msb0;
use bitvec::vec::BitVec;
use bitvec::view::BitView;

fn main() {
    use std::io::{Read, Write};
    let mut fd = std::fs::File::open("input").unwrap();
    let mut contents = String::new();
    fd.read_to_string(&mut contents).unwrap();
}

struct PacketStream {
    backing: BitVec<Msb0, u8>,
    idx: usize,
}

impl From<&str> for PacketStream {
    fn from(s: &str) -> Self {
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
        Self {
            backing: backing.view_bits::<Msb0>().to_owned(),
            idx: 0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hex_to_bin() {
        let s = "D2FE28";
        let bin: PacketStream = s.into();
        assert_eq!(vec![0xd2, 0xfe, 0x28], bin.backing.as_raw_slice());

        let s = "EE00D40C823060";
        let bin: PacketStream = s.into();
        assert_eq!(
            vec![0xEE, 0x00, 0xD4, 0x0C, 0x82, 0x30, 0x60],
            bin.backing.as_raw_slice()
        );
    }
}
