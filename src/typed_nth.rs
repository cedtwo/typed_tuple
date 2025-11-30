//! Submodule providing the `TypedNth` trait for accessing the nth element of
//! a tuple by index.

use crate::prelude::*;

/// Trait for accessing the nth element of a tuple by index.
pub trait TypedNth<Idx: typenum::Unsigned>:
    NthIndex<Idx> + TypedIndex<Idx, <Self as NthIndex<Idx>>::NthType>
{
}

impl<Idx: typenum::Unsigned, TT> TypedNth<Idx> for TT where
    TT: NthIndex<Idx> + TypedIndex<Idx, <TT as NthIndex<Idx>>::NthType>
{
}
