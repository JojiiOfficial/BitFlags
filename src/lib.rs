use std::{
    fmt::{Binary, Debug, Display},
    ops::{Add, AddAssign, BitAnd, BitOr, BitXor, Not, Shl},
};

#[derive(Clone, Copy, Default)]
pub struct BitFlag<T: BitflagAble> {
    val: T,
}

impl<T: BitflagAble> Debug for BitFlag<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl<T: BitflagAble> Display for BitFlag<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = self.val;
        write!(f, "{:b}", v)
    }
}

pub trait BitflagAble:
    BitOr<Output = Self>
    + PartialOrd
    + BitAnd<Output = Self>
    + BitXor<Output = Self>
    + Shl<Output = Self>
    + Not<Output = Self>
    + Add<Output = Self>
    + Default
    + Display
    + Binary
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
        + Add<Output = Self>
        + Default
        + Display
        + Binary
        + Copy
        + PartialEq
        + From<u8>
{
}

impl<T: BitflagAble> Add<Self> for BitFlag<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new_with_value(self.val.add(rhs.val))
    }
}

impl<T: BitflagAble, U: BitflagAble + Add<T, Output = T>> Add<U> for BitFlag<T> {
    type Output = Self;

    fn add(self, rhs: U) -> Self::Output {
        let v = rhs.add(self.val);
        Self::new_with_value(v)
    }
}

impl<T: BitflagAble, U: BitflagAble + Add<T, Output = T>> AddAssign<U> for BitFlag<T> {
    fn add_assign(&mut self, rhs: U) {
        self.val = rhs.add(self.val)
    }
}

impl<T: BitflagAble> AddAssign<Self> for BitFlag<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.val = self.val + rhs.val;
    }
}

impl<T: BitflagAble> From<T> for BitFlag<T> {
    fn from(t: T) -> Self {
        Self { val: t }
    }
}

impl<T: BitflagAble> BitFlag<T> {
    /// Creates a new BitFlag value
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new BitFlag with a set value
    pub fn new_with_value(val: T) -> Self {
        Self { val }
    }

    /// Sets a bit at the given `pos` to `val`
    pub fn set(&mut self, pos: T, val: bool) {
        // Check for overflow
        if Self::is_overflow(pos) {
            return;
        }

        self.set_unchecked(pos, val);
    }

    /// Sets a bit at the given `pos` to `val` without overflow checks
    pub fn set_unchecked(&mut self, pos: T, val: bool) {
        let mask = T::from(1_u8) << pos;

        if val {
            self.val = self.val | mask;
        } else {
            self.val = self.val & Self::invert(mask);
        }
    }

    /// Set the bitflags value from `start` to `end` (inclusive) to `val`[0..end-start+1]
    pub fn set_range<V: Into<BitFlag<T>>>(&mut self, range: (u8, u8), val: V) {
        if range.0 > range.1 || Self::is_overflow(T::from(range.1)) {
            return;
        }

        self.set_range_unchecked(range, val);
    }

    /// Set the bitflags value from `start` to `end` (inclusive) to `val`[0..end-start+1]
    pub fn set_range_unchecked<V: Into<BitFlag<T>>>(&mut self, range: (u8, u8), val: V) {
        let val = val.into();

        for (i, flag_pos) in (range.0..=range.1).enumerate() {
            println!("{:?} {:?}", i, flag_pos);
            self.set_unchecked(T::from(flag_pos), val.get_unchecked(T::from(i as u8)));
        }
    }

    /// Get the value between `start` and `end` as T
    pub fn get_range(&self, range: (u8, u8)) -> Option<T> {
        if range.0 > range.1 || Self::is_overflow(T::from(range.1)) {
            return None;
        }

        Some(self.get_range_unchecked(range))
    }

    /// Get the value between `start` and `end` as T unchecked
    pub fn get_range_unchecked(&self, range: (u8, u8)) -> T {
        let mut cpy: BitFlag<T> = BitFlag::new();

        for (i, flag_pos) in (range.0..=range.1).enumerate() {
            cpy.set_unchecked(T::from(i as u8), self.get_unchecked(T::from(flag_pos)));
        }

        cpy.val
    }

    /// Gets a bit at the given [`pos`]
    pub fn get(&self, pos: T) -> bool {
        if Self::is_overflow(pos) {
            return false;
        }

        self.get_unchecked(pos)
    }

    /// Gets a bit at the given [`pos`]
    pub fn get_unchecked(&self, pos: T) -> bool {
        let mask = T::from(1_u8) << pos;
        (self.val & mask) == mask
    }

    /// Get the raw value of the bitflag
    pub fn raw(&self) -> T {
        self.val
    }

    /// Clears the value to `T::default()`
    pub fn clear(&mut self) {
        self.val = T::default();
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

    #[test]
    fn test_set_range() {
        let mut bf: BitFlag<u32> = BitFlag::new();
        let max = u32::MAX;

        bf.set_range((3, 6), max);
        assert_eq!(bf.raw(), 0b01111000);

        bf.clear();

        bf.set_range((28, 31), max);
        assert_eq!(bf.raw(), 0b11110000000000000000000000000000);

        bf.clear();

        let e: u32 = 0b1011;
        let e: BitFlag<u32> = BitFlag::new_with_value(e);
        bf.set_range((28, 31), e);
        assert_eq!(bf.raw(), 0b10110000000000000000000000000000);
    }

    #[test]
    fn test_get_range() {
        let bf: BitFlag<u8> = BitFlag::new_with_value(0b101110);
        assert_eq!(bf.get_range((3, 0)), None);

        assert_eq!(bf.get_range((1, 2)), Some(0b11));

        assert_eq!(bf.get_range((0, 3)), Some(0b1110));

        assert_eq!(bf.get_range((0, 4)), Some(0b01110));

        assert_eq!(bf.get_range((4, 5)), Some(0b10));

        assert_eq!(bf.get_range((0, 5)), Some(0b101110));
    }
}
