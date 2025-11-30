//! Sub-module for type/index mapping between bounds.
use crate::prelude::*;

/// Trait for mapping a range of indices to a range of types.
pub trait TypedBounds<INDEX: TupleIndex, T>:
    TypedIndex<INDEX, T>
    + Sized
    + NthIndex<INDEX>
    + LastIndex<Last: TupleIndexSub<INDEX>>
    + NthIndexedUntil<INDEX>
{
    /// The type of the remaining tuple after popping element of type `T`.
    type PopOutput;
    /// The type of the left tuple when splitting exclusively (excludes element
    /// at INDEX): [.., INDEX).
    type SplitLeftExclusive: ChainRight<(T,), Output = Self::SplitLeftInclusive>
        + ChainRight<Self::SplitRightExclusive, Output = Self::PopOutput>
        + ChainRight<Self::SplitRightInclusive, Output = Self>;
    /// The type of the left tuple when splitting inclusively (includes element
    /// at INDEX): [.., INDEX].
    type SplitLeftInclusive: NthIndexedAs<INDEX, Self>
        + TypedUntil<INDEX>
        + NthIndexedUntil<INDEX>
        + TypedBounds<
            INDEX,
            T,
            SplitLeftExclusive = Self::SplitLeftExclusive,
            SplitRightInclusive = (T,),
        > + ChainRight<Self::SplitRightExclusive, Output = Self>;
    /// The type of the right tuple when splitting exclusively (excludes element
    /// at INDEX): (INDEX, ..].
    type SplitRightExclusive: ChainLeft<(T,), Output = Self::SplitRightInclusive>
        + ChainLeft<Self::SplitLeftExclusive, Output = Self::PopOutput>
        + ChainLeft<Self::SplitLeftInclusive, Output = Self>;
    /// The type of the right tuple when splitting inclusively (includes element
    /// at INDEX): [INDEX, ..].
    type SplitRightInclusive: TypedBounds<
            TupleIndex0,
            T,
            SplitRightExclusive = Self::SplitRightExclusive,
            SplitLeftInclusive = (T,),
        > + ChainLeft<Self::SplitLeftExclusive, Output = Self>;

    /// Splits the tuple exclusively at INDEX, returning the element and the
    /// surrounding tuples.
    ///
    /// # Returns
    ///
    /// A tuple containing (left_exclusive, element, right_exclusive) where:
    /// - `left_exclusive` contains elements [0..INDEX)
    /// - `element` is the element at INDEX
    /// - `right_exclusive` contains elements (INDEX..)
    ///
    /// # Example
    ///
    /// ```rust
    /// # use typed_tuple::prelude::*;
    /// let tuple = (1u8, 2u16, 3u32, 4u64, 5i8);
    /// let (left, element, right) = TypedBounds::<TupleIndex2, u32>::split_exclusive_at(tuple);
    /// assert_eq!(left, (1u8, 2u16));
    /// assert_eq!(element, 3u32);
    /// assert_eq!(right, (4u64, 5i8));
    /// ```
    fn split_exclusive_at(self) -> (Self::SplitLeftExclusive, T, Self::SplitRightExclusive);
}
