//! Sub-module for tuple index related traits.

use crate::prelude::*;

/// Trait to identify the type at a specific index of a tuple.
///
/// This trait is implemented for all tuple types (by default up to size 64,
/// and up to 128 with the `len_128` feature) for valid indices and provides
/// an associated type `NthType` for the type of the element at index `Idx`.
///
/// # Examples
///
/// ```rust
/// # use typed_tuple::prelude::*;
/// // Get the type at index 1
/// type MyTuple = (u8, u16, u32);
/// type SecondType = <MyTuple as NthIndex<TupleIndex1>>::NthType;
///
/// let tuple: MyTuple = (1, 2, 3);
/// let second: &SecondType = TypedTuple::<TupleIndex1, SecondType>::get(&tuple);
/// assert_eq!(*second, 2u16);
///
/// // Works with different indices
/// type ThirdType = <MyTuple as NthIndex<TupleIndex2>>::NthType;
/// let third: &ThirdType = TypedTuple::<TupleIndex2, ThirdType>::get(&tuple);
/// assert_eq!(*third, 3u32);
/// ```
pub trait NthIndex<Idx: TupleIndex> {
    /// The type of the element at index `Idx` in the tuple.
    type NthType;
}
