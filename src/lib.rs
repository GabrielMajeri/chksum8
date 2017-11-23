//! Sum all the bytes of a data structure or an array.
//!
//! A commonly used method to ensure the validity of data is to calculate
//! the sum of all the bytes, and compare it to a known value.
//!
//! **Note**: if you need a way to ensure data integrity, use some better algorithm like CRC-32.
//! This crate is only intended to be used with legacy APIs.

#![no_std]

#![deny(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", deny(clippy))]

use core::{mem, slice};

/// Sums all the bytes of a data structure.
pub fn sum<T>(data: &T) -> u8 {
    let ptr = data as *const _ as *const u8;
    let len = mem::size_of::<T>();

    let data = unsafe { slice::from_raw_parts(ptr, len) };

    sum_slice(data)
}

/// Sums all the bytes in an array.
pub fn sum_slice(data: &[u8]) -> u8 {
    data.iter().fold(0, |a, &b| a.wrapping_add(b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_sum() {
        struct Simple(u32, u32);

        let simple = Simple(0xAA_00_BB_00, 0xAA_00_00_00);

        assert_eq!(sum(&simple), 15);
    }

    #[test]
    fn array_sum() {
        let data: &[u8] = &[1, 0, 8, 24, 45, 192, 25, 253, 0];

        assert_eq!(sum_slice(&data), 36);
    }

    #[test]
    fn zero_size_type() {
        struct Zero;

        assert_eq!(sum(&Zero), 0);
    }
}
