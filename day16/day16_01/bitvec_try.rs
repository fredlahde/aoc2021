use bitvec::order::BitOrder;
use bitvec::prelude::Msb0;
use bitvec::ptr::BitRef;
use bitvec::ptr::Mutability;
use bitvec::ptr::{Const, Mut};
use bitvec::slice::BitSlice;
use bitvec::store::BitStore;

fn main() {
    use bitvec::prelude::*;

    let mut bv = bitvec![0, 0, 1];
    assert_eq!(1, convert(bv.iter()).unwrap());

    let mut bv = bitvec![0, 0, 0];
    assert_eq!(0, convert(bv.iter()).unwrap());

    let mut bv = bitvec![1, 1, 1];
    assert_eq!(7, convert(bv.iter()).unwrap());

    assert_eq!(3, convert(bv[1..=2].iter()).unwrap());

    let bits = vec![0xa, 0xbu8];
    let mut bv = bits.view_bits::<Msb0>();
    println!("{:?}", bv);
    assert_eq!(171, convert(bv.iter()).unwrap());
}

use std::cmp::PartialEq;
use std::ops::BitXor;
use std::ops::Shl;

#[derive(Debug)]
pub enum ConversionError {
    Overflow,
}

#[cfg(debug_assertions)]
pub fn convert<
    'a,
    M: Mutability,
    O: BitOrder,
    V: BitStore,
    I: Iterator<Item = BitRef<'a, M, O, V>>,
    T: From<bool> + BitXor<Output = T> + Shl<Output = T> + std::fmt::Debug + std::fmt::Binary,
>(
    bits: I,
) -> Result<T, ConversionError> {
    let have_space = std::mem::size_of::<T>() * 8;

    let mut count = 0;
    let ret = bits.fold(T::from(false), |result, bit| {
        count += 1;
        println!("{:?}, {:b}", T::from(*bit), result);
        (result << T::from(true)) ^ T::from(*bit)
    });
    if count > have_space {
        Err(ConversionError::Overflow)
    } else {
        Ok(ret)
    }
}

#[cfg(not(debug_assertions))]
pub fn convert<
    'a,
    I: Iterator<Item = BitRef<'a, Const>>,
    T: From<bool> + BitXor<Output = T> + Shl<Output = T> + std::fmt::Debug,
>(
    bits: I,
) -> Result<T, ConversionError> {
    let have_space = std::mem::size_of::<T>() * 8;

    let ret = bits.fold(T::from(false), |result, bit| {
        (result << T::from(true)) ^ T::from(*bit)
    });
    Ok(ret)
}
