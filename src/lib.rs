#![no_std]

use core::{mem, slice};

/// Sums all the bytes of a data structure using wrapping operations.
pub fn checksum8<T>(data: &T) -> u8 {
     let data = unsafe {
       let ptr = mem::transmute(data);
        let len = mem::size_of::<T>();
        slice::from_raw_parts(ptr, len)
    };

    checksum8_raw(data)
}

/// Sums all the bytes in an array, modulo 256.
pub fn checksum8_raw(data: &[u8]) -> u8 {
    data.iter().fold(0, |a, &b| a.wrapping_add(b))
}

// TODO: write unit tests.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_sum() {
        struct Simple(u32, u32);

        let simple = Simple(0xAA_00_BB_00, 0xAA_00_00_00);

        assert_eq!(checksum8(&simple), 15);
    }

    #[test]
    fn array_sum() {
        let data: &[u8] = &[1, 0, 8, 24, 45, 192, 25, 253, 0];

        assert_eq!(checksum8_raw(&data), 36);
    }

    #[test]
    fn zero_size_type() {
        struct Zero;

        assert_eq!(checksum8(&Zero), 0);
    }
}
