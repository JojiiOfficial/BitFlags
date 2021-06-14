use std::ops::{BitAnd, BitOr, BitXor, Not, Shl};

#[derive(Clone, Copy, Default)]
pub struct BitFlag<T: BitflagAble> {
    val: T,
}

pub trait BitflagAble:
    BitOr<Output = Self>
    + BitAnd<Output = Self>
    + BitXor<Output = Self>
    + Shl<Output = Self>
    + Not<Output = Self>
    + Default
    + Copy
    + PartialEq
    + From<u8>
{
}

impl<U> BitflagAble for U where
    U: BitOr<Output = U>
        + BitAnd<Output = U>
        + BitXor<Output = U>
        + Shl<Output = U>
        + Not<Output = U>
        + Default
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
        let mask = T::from(1_u8) << pos;

        if val {
            self.val = self.val | mask;
        } else {
            self.val = self.val & Self::invert(mask);
        }
    }

    /// Gets a bit at the given [`pos`]
    pub fn get(&self, pos: T) -> bool {
        let mask = T::from(1_u8) << pos;
        (self.val & mask) == mask
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
