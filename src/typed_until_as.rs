//! Submodule providing the `NthIndexedUntil` and `NthIndexedAs` traits for
//! ensuring matching types between tuples up to a specific index.

use crate::prelude::*;

typed_tuple_macros::define_nth_indexed_until_trait!();

typed_tuple_macros::impl_nth_indexed_until_trait!();

typed_tuple_macros::define_nth_indexed_as_trait!();

typed_tuple_macros::impl_nth_indexed_as_trait!();
