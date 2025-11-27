//! Submodule providing the `TypedLast` trait for accessing the last element of
//! a tuple by type.

use crate::prelude::*;

/// Trait for accessing the last element of a tuple by type.
///
/// This trait is implemented for tuples where the last element is of type `T`.
/// It combines the functionality of `LastIndex` and `TypedUntil` to provide
/// type-safe access to the last element and all elements up to it.
///
/// # Examples
///
/// ```rust
/// # use typed_tuple::prelude::*;
/// fn get_last<TT: TypedLast<u32>>(tuple: &TT) -> &u32 {
///     tuple.get()
/// }
///
/// let tuple = (1u8, 2u16, 3u32);
/// assert_eq!(*get_last(&tuple), 3u32);
///
/// let tuple2 = ("hello", 'x', 42u32);
/// assert_eq!(*get_last(&tuple2), 42u32);
/// ```
pub trait TypedLast<T>:
    LastIndex<NthType = T>
    + TypedUntil<<Self as LastIndex>::Last>
    + IndexedTuple<
        <Self as LastIndex>::Last,
        T,
        SplitRightInclusive = (T,),
        SplitLeftInclusive = Self,
        SplitRightExclusive = (),
    >
{
}

impl<T, TT> TypedLast<T> for TT where
    TT: LastIndex<NthType = T>
        + TypedUntil<<TT as LastIndex>::Last>
        + IndexedTuple<
            <TT as LastIndex>::Last,
            T,
            SplitRightInclusive = (T,),
            SplitLeftInclusive = TT,
            SplitRightExclusive = (),
        >
{
}
