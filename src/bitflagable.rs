use std::{
    fmt::{Binary, Display},
    ops::{Add, BitAnd, BitOr, BitXor, Not, Shl, Shr},
};

/// Trait defining everything that can be used as bitflag base type.
pub trait BitflagAble:
    BitOr<Output = Self>
    + PartialOrd
    + BitAnd<Output = Self>
    + BitXor<Output = Self>
    + Shl<Output = Self>
    + Shr<Output = Self>
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
        + Shr<Output = Self>
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
