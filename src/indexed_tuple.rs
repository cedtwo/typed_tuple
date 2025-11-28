//! Sub-module for tuple index related traits.

use crate::prelude::*;

/// Trait for mapping an index (or indices) to a type (or types).
pub trait IndexedTuple<INDEX: TupleIndex, T>:
    Sized + NthIndex<INDEX> + LastIndex<Last: TupleIndexSub<INDEX>> + NthIndexedUntil<INDEX>
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
        + IndexedTuple<
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
    type SplitRightInclusive: IndexedTuple<
            TupleIndex0,
            T,
            SplitRightExclusive = Self::SplitRightExclusive,
            SplitLeftInclusive = (T,),
        > + ChainLeft<Self::SplitLeftExclusive, Output = Self>;

    /// Get a reference to the element of type `T`.
    /// # Example
    /// ```
    /// # use typed_tuple::prelude::*;
    /// // Get by type.
    /// let tuple = ("a", 'b', 2usize);
    /// let a: &&str = tuple.get_at();
    /// let b: &char = tuple.get_at();
    /// let c: &usize = tuple.get_at();
    ///
    /// // Get by 'const' index.
    /// let a = IndexedTuple::<TupleIndex0, _>::get_at(&tuple);
    /// let b = IndexedTuple::<TupleIndex1, _>::get_at(&tuple);
    /// let c = IndexedTuple::<TupleIndex2, _>::get_at(&tuple);
    /// ```
    fn get_at(&self) -> &T;

    /// Get a mutable reference to the element of type `T`.
    /// # Example
    /// ```
    /// # use typed_tuple::prelude::*;
    /// // Mutate by type.
    /// let mut tuple = ("a", 'b', 2usize);
    /// *tuple.get_mut_at() = "c";
    /// *tuple.get_mut_at() = 'd';
    /// *tuple.get_mut_at() = 3usize;
    /// assert_eq!(tuple, ("c", 'd', 3));
    ///
    /// // Mutate by 'const' index.
    /// *IndexedTuple::<TupleIndex0, _>::get_mut_at(&mut tuple) = "e";
    /// *IndexedTuple::<TupleIndex1, _>::get_mut_at(&mut tuple) = 'f';
    /// *IndexedTuple::<TupleIndex2, _>::get_mut_at(&mut tuple) = 4usize;
    /// assert_eq!(tuple, ("e", 'f', 4))
    /// ```
    fn get_mut_at(&mut self) -> &mut T;

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
    /// let (left, element, right) = IndexedTuple::<TupleIndex2, u32>::split_exclusive_at(tuple);
    /// assert_eq!(left, (1u8, 2u16));
    /// assert_eq!(element, 3u32);
    /// assert_eq!(right, (4u64, 5i8));
    /// ```
    fn split_exclusive_at(self) -> (Self::SplitLeftExclusive, T, Self::SplitRightExclusive);
}
