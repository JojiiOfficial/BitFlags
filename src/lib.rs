use std::{
    fmt::Display,
    ops::{BitAnd, BitOr, BitXor, Not, Shl},
};

#[derive(Clone, Copy, Default)]
pub struct BitFlag<T: BitflagAble> {
    val: T,
}

pub trait BitflagAble:
    BitOr<Output = Self>
    + PartialOrd
    + BitAnd<Output = Self>
    + BitXor<Output = Self>
    + Shl<Output = Self>
    + Not<Output = Self>
    + Default
    + Display
    + Copy
    + PartialEq
    + From<u8>
{
}

impl<U> BitflagAble for U where
    U: BitOr<Output = U>
        + PartialOrd
        + BitAnd<Output = U>
        + BitXor<Output = U>
        + Shl<Output = U>
        + Not<Output = U>
        + Default
        + Display
        + Copy
        + PartialEq
        + From<u8>
{
}

impl<T: BitflagAble> BitFlag<T> {
    /// Creates a new BitFlag value
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets a bit at the given `pos` to `val`
    pub fn set(&mut self, pos: T, val: bool) {
        if Self::is_overflow(pos) {
            return;
        }

        // Check for overflow
        let size = T::from((std::mem::size_of::<T>() * 8) as u8);
        if pos >= size {
            return;
        }

        let mask = T::from(1_u8) << pos;

        if val {
            self.val = self.val | mask;
        } else {
            self.val = self.val & Self::invert(mask);
        }
    }

    /// Gets a bit at the given [`pos`]
    pub fn get(&self, pos: T) -> bool {
        if Self::is_overflow(pos) {
            return false;
        }

        let mask = T::from(1_u8) << pos;
        (self.val & mask) == mask
    }

    /// Returns true if [`pos`] would cause an overflow
    pub fn is_overflow(pos: T) -> bool {
        let size = T::from((std::mem::size_of::<T>() * 8) as u8);
        pos >= size
    }

    ///  Inverts all bits in [`val`]
    fn invert(val: T) -> T {
        (!val) as T
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_overflow() {
        let mut bf8: BitFlag<u8> = BitFlag::new();
        // Set won't do anything
        bf8.set(8, true);
        assert_eq!(bf8.get(8), false);
    }

    #[test]
    fn set_all_false_u8() {
        let mut bf8: BitFlag<u8> = BitFlag::new();
        for i in 0..7 {
            bf8.set(i, false);
            assert_eq!(bf8.get(i), false);
        }
    }

    #[test]
    fn set_all_true_u8() {
        let mut bf8: BitFlag<u8> = BitFlag::new();
        for i in 0..7 {
            bf8.set(i, true);
            assert_eq!(bf8.get(i), true);
        }
    }

    #[test]
    fn toggle_all_u8() {
        let mut bf8: BitFlag<u8> = BitFlag::new();
        for i in 0..7 {
            bf8.set(i, true);
            assert_eq!(bf8.get(i), true);

            bf8.set(i, false);
            assert_eq!(bf8.get(i), false);
        }
    }

    #[test]
    fn set_all_false_u32() {
        let mut bf8: BitFlag<u32> = BitFlag::new();
        for i in 0..31 {
            bf8.set(i, false);
            assert_eq!(bf8.get(i), false);
        }
    }

    #[test]
    fn set_all_true_u32() {
        let mut bf8: BitFlag<u32> = BitFlag::new();
        for i in 0..31 {
            bf8.set(i, true);
            assert_eq!(bf8.get(i), true);
        }
    }

    #[test]
    fn toggle_all_u32() {
        let mut bf8: BitFlag<u32> = BitFlag::new();
        for i in 0..31 {
            bf8.set(i, true);
            assert_eq!(bf8.get(i), true);

            bf8.set(i, false);
            assert_eq!(bf8.get(i), false);
        }
    }
}
