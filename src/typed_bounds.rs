//! Sub-module for type/index mapping between bounds.
use crate::prelude::*;
use core::ops::Sub;

/// Trait for mapping a range of indices to a range of types.
pub trait TypedBounds<Idx: typenum::Unsigned, T>:
    TypedIndex<Idx, T> + Sized + NthIndex<Idx> + LastIndex<Last: Sub<Idx>>
{
    /// The type of the remaining tuple after popping element of type `T`.
    type PopOutput;
    /// The type of the left tuple when splitting exclusively (excludes element
    /// at Idx): [.., Idx).
    type SplitLeftExclusive: ChainRight<(T,), Output = Self::SplitLeftInclusive>
        + ChainRight<Self::SplitRightExclusive, Output = Self::PopOutput>
        + ChainRight<Self::SplitRightInclusive, Output = Self>;
    /// The type of the left tuple when splitting inclusively (includes element
    /// at Idx): [.., Idx].
    type SplitLeftInclusive: TypedBounds<
            Idx,
            T,
            SplitLeftExclusive = Self::SplitLeftExclusive,
            SplitRightInclusive = (T,),
        > + ChainRight<Self::SplitRightExclusive, Output = Self>;
    /// The type of the right tuple when splitting exclusively (excludes element
    /// at Idx): (Idx, ..].
    type SplitRightExclusive: ChainLeft<(T,), Output = Self::SplitRightInclusive>
        + ChainLeft<Self::SplitLeftExclusive, Output = Self::PopOutput>
        + ChainLeft<Self::SplitLeftInclusive, Output = Self>;
    /// The type of the right tuple when splitting inclusively (includes element
    /// at Idx): [Idx, ..].
    type SplitRightInclusive: TypedBounds<
            typenum::U0,
            T,
            SplitRightExclusive = Self::SplitRightExclusive,
            SplitLeftInclusive = (T,),
        > + ChainLeft<Self::SplitLeftExclusive, Output = Self>;

    /// Splits the tuple exclusively at Idx, returning the element and the
    /// surrounding tuples.
    ///
    /// # Returns
    ///
    /// A tuple containing (left_exclusive, element, right_exclusive) where:
    /// - `left_exclusive` contains elements [0..Idx)
    /// - `element` is the element at Idx
    /// - `right_exclusive` contains elements (Idx..)
    ///
    /// # Example
    ///
    /// ```rust
    /// # use typed_tuple::prelude::*;
    /// let tuple = (1u8, 2u16, 3u32, 4u64, 5i8);
    /// let (left, element, right) = TypedBounds::<typenum::U2, u32>::split_exclusive_at(tuple);
    /// assert_eq!(left, (1u8, 2u16));
    /// assert_eq!(element, 3u32);
    /// assert_eq!(right, (4u64, 5i8));
    /// ```
    fn split_exclusive_at(self) -> (Self::SplitLeftExclusive, T, Self::SplitRightExclusive);
}
