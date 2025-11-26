//! Module defining the `LastIndex` trait for tuples.

use crate::prelude::*;

/// Trait to identify the last element index of a tuple.
///
/// This trait is implemented for many tuple types (by default up to size 64,
/// and up to 128 with the `len_128` feature) and provides an associated
/// type `Last` that corresponds to the appropriate `TupleIndexN` marker for
/// the tuple's last element.
///
/// # Examples
///
/// ```rust
/// # use typed_tuple::prelude::*;
/// // Access last element using LastIndex
/// type MyTuple = (u8, u16, u32);
/// type LastIdx = <MyTuple as LastIndex>::Last;
///
/// let tuple: MyTuple = (1, 2, 3);
/// let last: &u32 = TypedTuple::<LastIdx, u32>::get(&tuple);
/// assert_eq!(*last, 3u32);
///
/// // Works with different tuple sizes
/// type BiggerTuple = (u8, u16, u32, u64, i8);
/// type BiggerLast = <BiggerTuple as LastIndex>::Last;
///
/// let tuple2: BiggerTuple = (1, 2, 3, 4, 5);
/// let last2: &i8 = TypedTuple::<BiggerLast, i8>::get(&tuple2);
/// assert_eq!(*last2, 5i8);
/// ```
pub trait LastIndex: NthIndex<Self::Last> {
    /// The index marker type for the last element of this tuple.
    type Last: TupleIndex;
}
