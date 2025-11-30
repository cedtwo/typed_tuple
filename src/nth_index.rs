//! Sub-module for tuple index related traits.

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
/// type SecondType = <MyTuple as NthIndex<typenum::U1>>::NthType;
///
/// let tuple: MyTuple = (1, 2, 3);
/// let second: &SecondType = tuple.get::<typenum::U1>();
/// assert_eq!(*second, 2u16);
///
/// // Works with different indices
/// type ThirdType = <MyTuple as NthIndex<typenum::U2>>::NthType;
/// let third: &ThirdType = tuple.get::<typenum::U2>();
/// assert_eq!(*third, 3u32);
/// ```
pub trait NthIndex<Idx: typenum::Unsigned> {
    /// The type of the element at index `Idx` in the tuple.
    type NthType;
}
