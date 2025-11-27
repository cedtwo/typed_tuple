//! Sub-module for tuple index related traits.

use crate::prelude::*;

/// Trait for tuple element manipulation by type.
pub trait TypedTuple<INDEX: TupleIndex, T>:
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
        + TypedTuple<
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
    type SplitRightInclusive: TypedTuple<
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
    /// let a: &&str = tuple.get();
    /// let b: &char = tuple.get();
    /// let c: &usize = tuple.get();
    ///
    /// // Get by 'const' index.
    /// let a = TypedTuple::<TupleIndex0, _>::get(&tuple);
    /// let b = TypedTuple::<TupleIndex1, _>::get(&tuple);
    /// let c = TypedTuple::<TupleIndex2, _>::get(&tuple);
    /// ```
    fn get(&self) -> &T;

    /// Get a mutable reference to the element of type `T`.
    /// # Example
    /// ```
    /// # use typed_tuple::prelude::*;
    /// // Mutate by type.
    /// let mut tuple = ("a", 'b', 2usize);
    /// *tuple.get_mut() = "c";
    /// *tuple.get_mut() = 'd';
    /// *tuple.get_mut() = 3usize;
    /// assert_eq!(tuple, ("c", 'd', 3));
    ///
    /// // Mutate by 'const' index.
    /// *TypedTuple::<TupleIndex0, _>::get_mut(&mut tuple) = "e";
    /// *TypedTuple::<TupleIndex1, _>::get_mut(&mut tuple) = 'f';
    /// *TypedTuple::<TupleIndex2, _>::get_mut(&mut tuple) = 4usize;
    /// assert_eq!(tuple, ("e", 'f', 4))
    /// ```
    fn get_mut(&mut self) -> &mut T;

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
    /// let (left, element, right) = TypedTuple::<TupleIndex2, u32>::split_exclusive(tuple);
    /// assert_eq!(left, (1u8, 2u16));
    /// assert_eq!(element, 3u32);
    /// assert_eq!(right, (4u64, 5i8));
    /// ```
    fn split_exclusive(self) -> (Self::SplitLeftExclusive, T, Self::SplitRightExclusive);
}
