//! Submodule for TypedTuple extension traits.

use crate::prelude::*;

/// Extension trait to add additional methods to TypedTuple.
pub trait TypedTupleExt<T>: Sized {
    #[inline]
    /// Pops the element of type `T` from the tuple, returning it along with
    /// the remaining tuple.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use typed_tuple::prelude::*;
    /// let mut tuple = (1u8, 2u16, 3u32);
    /// let (val, rest) = tuple.pop_at::<TupleIndex2>();
    /// assert_eq!(val, 3u32);
    /// assert_eq!(rest, (1u8, 2u16));
    /// ```
    fn pop_at<Idx>(self) -> (T, Self::PopOutput)
    where
        Idx: TupleIndex,
        Self: TypedTuple<Idx, T>,
    {
        <Self as TypedTuple<Idx, T>>::pop(self)
    }

    #[inline]
    /// Swaps the element of type `T` at Idx with the element at Other.
    ///
    /// Both indices must contain elements of type `T`. If Idx == Other,
    /// this is a no-op.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use typed_tuple::prelude::*;
    /// let mut tuple = (1u32, "hello", 2u32, 'x', 3u32);
    /// tuple.swap_at::<TupleIndex0, TupleIndex2>();
    /// assert_eq!(tuple, (2u32, "hello", 1u32, 'x', 3u32));
    /// ```
    fn swap_at<Idx, Other>(&mut self)
    where
        Self: TypedTuple<Idx, T> + TypedTuple<Other, T>,
        Idx: TupleIndex,
        Other: TupleIndex,
    {
        <Self as TypedTuple<Idx, T>>::swap::<Other>(self);
    }

    #[inline]
    /// Splits the tuple at Idx (inclusive left), returning two tuples.
    ///
    /// The element at Idx is included in the left tuple.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use typed_tuple::prelude::*;
    /// let tuple = (1u8, 2u16, 3u32, 4u64, 5i8);
    /// let (left, right) = tuple.split_left_at::<TupleIndex2>();
    /// assert_eq!(left, (1u8, 2u16, 3u32));
    /// assert_eq!(right, (4u64, 5i8));
    /// ```
    fn split_left_at<Idx>(self) -> (Self::SplitLeftInclusive, Self::SplitRightExclusive)
    where
        Idx: TupleIndex,
        Self: TypedTuple<Idx, T>,
    {
        <Self as TypedTuple<Idx, T>>::split_left(self)
    }

    #[inline]
    /// Splits the tuple at Idx (inclusive right), returning two tuples.
    ///
    /// The element at Idx is included in the right tuple.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use typed_tuple::prelude::*;
    /// let tuple = (1u8, 2u16, 3u32, 4u64, 5i8);
    /// let (left, right) = tuple.split_right_at::<TupleIndex2>();
    /// assert_eq!(left, (1u8, 2u16));
    /// assert_eq!(right, (3u32, 4u64, 5i8));
    /// ```
    fn split_right_at<Idx>(self) -> (Self::SplitLeftExclusive, Self::SplitRightInclusive)
    where
        Idx: TupleIndex,
        Self: TypedTuple<Idx, T>,
    {
        <Self as TypedTuple<Idx, T>>::split_right(self)
    }

    #[inline]
    /// Splits the tuple at Idx (inclusive both sides), returning two tuples.
    ///
    /// The element at Idx is cloned and included in both tuples.
    ///
    /// # Example
    /// ```
    /// # use typed_tuple::prelude::*;
    /// let tuple = (1u8, 2u16, 3u32, 4u64, 5i8);
    /// let (left, right) = tuple.split_inclusive_at::<TupleIndex2>();
    /// assert_eq!(left, (1u8, 2u16, 3u32));
    /// assert_eq!(right, (3u32, 4u64, 5i8));
    /// ```
    fn split_inclusive_at<Idx>(self) -> (Self::SplitLeftInclusive, Self::SplitRightInclusive)
    where
        Idx: TupleIndex,
        Self: TypedTuple<Idx, T>,
        T: Clone,
    {
        <Self as TypedTuple<Idx, T>>::split_inclusive(self)
    }
}

impl<T, TT> TypedTupleExt<T> for TT {}
