//! Submodule providing the `TypedNth` trait for accessing the nth element of
//! a tuple by index.

use crate::prelude::*;

/// Trait for accessing the nth element of a tuple by index.
pub trait TypedNth<Idx: TupleIndex>:
    NthIndex<Idx> + IndexedTuple<Idx, <Self as NthIndex<Idx>>::NthType>
{
}

impl<Idx: TupleIndex, TT> TypedNth<Idx> for TT where
    TT: NthIndex<Idx> + IndexedTuple<Idx, <TT as NthIndex<Idx>>::NthType>
{
}
