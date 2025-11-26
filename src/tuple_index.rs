//! Submodule defining the `TupleIndex` trait for tuple index types.

use crate::prelude::*;

/// Trait for tuple index types.
pub trait TupleIndex:
    Sized
    + TupleIndexSub<Self, Output = TupleIndex0>
    + TupleIndexSub<TupleIndex0, Output = Self>
    + TupleIndexAdd<TupleIndex0, Output = Self>
{
    /// The associated index value.
    const INDEX: usize;
}
