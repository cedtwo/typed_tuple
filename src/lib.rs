#![doc = include_str!("../README.md")]
#![no_std]
#![cfg_attr(feature = "len_128", recursion_limit = "256")]

pub mod chain;
pub mod last_index;
pub mod nth_index;
pub mod tuple_key;
pub mod typed_bounds;
pub mod typed_first;
pub mod typed_index;
pub mod typed_last;
pub mod typed_nth;
pub mod typed_tuple;

/// Prelude module re-exporting commonly used traits and types.
pub mod prelude {
    pub use crate::{
        chain::{ChainLeft, ChainRight},
        last_index::LastIndex,
        nth_index::NthIndex,
        tuple_key::TupleKey,
        typed_bounds::TypedBounds,
        typed_first::TypedFirst,
        typed_index::TypedIndex,
        typed_last::TypedLast,
        typed_nth::TypedNth,
        typed_tuple::TypedTuple,
    };

    // Re-export typenum for convenience
    pub use typenum::*;

    typed_tuple_macros::generate_last_index_impls!();
    typed_tuple_macros::generate_nth_index_impls!();
    typed_tuple_macros::generate_typed_tuple_impls!();
}
