//! Submodule providing the `TypedFirst` trait for accessing the first element
//! of a tuple by type.

use crate::prelude::*;

/// Trait for accessing the first element of a tuple by type.
///
/// This trait is implemented for tuples where the first element is of type `T`.
/// It combines the functionality of `FirstIndex` and `TypedTuple` to provide
/// type-safe access to the first element.
///
/// # Examples
///
/// ```rust
/// # use typed_tuple::prelude::*;
/// fn get_first<TT: TypedFirst<u32>>(tuple: &TT) -> &u32 {
///     tuple.get()
/// }
///
/// let tuple = (3u32, 2u16, 1u8);
/// assert_eq!(*get_first(&tuple), 3u32);
///
/// let tuple2 = (42u32, "hello", 'x');
/// assert_eq!(*get_first(&tuple2), 42u32);
/// ```
pub trait TypedFirst<T>:
    NthIndex<TupleIndex0, NthType = T>
    + TypedIndex<
        TupleIndex0,
        T,
        SplitLeftInclusive = (T,),
        SplitLeftExclusive = (),
        SplitRightInclusive = Self,
    >
{
}

impl<T, TT> TypedFirst<T> for TT where
    TT: NthIndex<TupleIndex0, NthType = T>
        + TypedIndex<
            TupleIndex0,
            T,
            SplitLeftInclusive = (T,),
            SplitLeftExclusive = (),
            SplitRightInclusive = TT,
        >
{
}
