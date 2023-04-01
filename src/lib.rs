pub mod bitflagable;

use bitflagable::BitflagAble;
use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign},
};

/// Wrapper for any type T that can be used for bitflags.
#[derive(Clone, Copy, Default)]
pub struct BitFlag<T> {
    val: T,
}

impl<T: BitflagAble> BitFlag<T> {
    /// Creates a new BitFlag value
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new BitFlag with a set value
    #[inline]
    pub fn new_with_value(val: T) -> Self {
        Self { val }
    }

    /// Sets a bit at the given `pos` to `val`
    #[inline]
    pub fn set(&mut self, pos: T, val: bool) {
        // Check for overflow
        if Self::is_overflow(pos) {
            return;
        }

        self.set_unchecked(pos, val);
    }

    /// Sets a bit at the given `pos` to `val` without overflow checks
    #[inline]
    pub fn set_unchecked(&mut self, pos: T, val: bool) {
        let mask = T::from(1_u8) << pos;

        if val {
            self.val = self.val | mask;
        } else {
            self.val = self.val & Self::invert(mask);
        }
    }

    /// Set the bitflags value from `start` to `end` (inclusive) to `val`[0..end-start+1]
    #[inline]
    pub fn set_range<V: Into<BitFlag<T>>>(&mut self, range: (u8, u8), val: V) {
        if range.0 > range.1 || Self::is_overflow(T::from(range.1)) {
            return;
        }

        self.set_range_unchecked(range, val);
    }

    /// Set the bitflags value from `start` to `end` (inclusive) to `val`[0..end-start+1]
    #[inline]
    pub fn set_range_unchecked<V: Into<BitFlag<T>>>(&mut self, range: (u8, u8), val: V) {
        let val = val.into();

        for (i, flag_pos) in (range.0..=range.1).enumerate() {
            self.set_unchecked(T::from(flag_pos), val.get_unchecked(T::from(i as u8)));
        }
    }

    /// Get the value between `start` and `end` as T
    #[inline]
    pub fn get_range(&self, range: (u8, u8)) -> Option<T> {
        if range.0 > range.1 || Self::is_overflow(T::from(range.1)) {
            return None;
        }

        Some(self.get_range_unchecked(range))
    }

    /// Get the value between `start` and `end` as T unchecked
    #[inline]
    pub fn get_range_unchecked(&self, range: (u8, u8)) -> T {
        let mut cpy: BitFlag<T> = BitFlag::new();

        for (i, flag_pos) in (range.0..=range.1).enumerate() {
            cpy.set_unchecked(T::from(i as u8), self.get_unchecked(T::from(flag_pos)));
        }

        cpy.val
    }

    /// Gets a bit at the given [`pos`]
    #[inline]
    pub fn get(&self, pos: T) -> bool {
        if Self::is_overflow(pos) {
            return false;
        }

        self.get_unchecked(pos)
    }

    /// Gets a bit at the given [`pos`]
    #[inline]
    pub fn get_unchecked(&self, pos: T) -> bool {
        let mask = T::from(1u8) << pos;
        (self.val & mask) != T::from(0u8)
    }

    /// Get the raw value of the bitflag
    #[inline]
    pub fn raw(&self) -> T {
        self.val
    }

    /// Clears the value to `T::default()`
    #[inline]
    pub fn clear(&mut self) {
        self.val = T::default();
    }

    /// Returns true if [`pos`] would cause an overflow
    #[inline]
    pub fn is_overflow(pos: T) -> bool {
        pos >= T::from(Self::size() as u8)
    }

    /// Returns the amonut of bits set
    #[inline]
    pub fn len(&self) -> usize {
        (0..Self::size())
            .filter(|i| self.get_unchecked(T::from(*i as u8)))
            .count()
    }

    /// Returns `true` if there is no bit set.
    #[inline]
    pub fn is_empty(&self) -> bool {
        T::from(0u8) == self.val
    }

    /// Returns an iterator over all fields of the bitflag.
    #[inline]
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = bool> + 'a {
        (0..Self::size()).map(move |i| self.get_unchecked(T::from(i as u8)))
    }

    /// Returns the amonut of bits that can be accessed for the given base type T
    #[inline]
    pub fn size() -> usize {
        std::mem::size_of::<T>() * 8
    }

    ///  Inverts all bits in [`val`]
    #[inline]
    fn invert(val: T) -> T {
        (!val) as T
    }
}

impl<T: BitflagAble> Debug for BitFlag<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl<T: BitflagAble> Display for BitFlag<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:b}", self.val)
    }
}

impl<T: BitflagAble> Add<Self> for BitFlag<T> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::new_with_value(self.val.add(rhs.val))
    }
}

impl<T: BitflagAble, U: BitflagAble + Add<T, Output = T>> Add<U> for BitFlag<T> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: U) -> Self::Output {
        let v = rhs.add(self.val);
        Self::new_with_value(v)
    }
}

impl<T: BitflagAble, U: BitflagAble + Add<T, Output = T>> AddAssign<U> for BitFlag<T> {
    #[inline]
    fn add_assign(&mut self, rhs: U) {
        self.val = rhs.add(self.val)
    }
}

impl<T: BitflagAble> AddAssign<Self> for BitFlag<T> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.val = self.val + rhs.val;
    }
}

impl<T: BitflagAble> From<T> for BitFlag<T> {
    #[inline]
    fn from(t: T) -> Self {
        Self { val: t }
    }
}

#[cfg(feature = "with_serde")]
impl<T> serde::Serialize for BitFlag<T>
where
    T: serde::Serialize,
{
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.val.serialize(serializer)
    }
}

#[cfg(feature = "with_serde")]
impl<'a, T> serde::Deserialize<'a> for BitFlag<T>
where
    T: serde::Deserialize<'a>,
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        let val = T::deserialize(deserializer)?;
        Ok(BitFlag { val })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_overflow() {
        let mut bf8: BitFlag<u8> = BitFlag::new();
        assert!(bf8.is_empty());
        // Set won't do anything as we're overflowing on purpose
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
        assert!(bf8.is_empty());
    }

    #[test]
    fn set_all_true_u8() {
        let mut bf8: BitFlag<u8> = BitFlag::new();
        for i in 0..7 {
            bf8.set(i, true);
            assert_eq!(bf8.get(i), true);
        }
        assert!(!bf8.is_empty());
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
        assert!(bf8.is_empty());
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
        assert!(bf8.is_empty());
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
        assert!(!bf.is_empty());
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
