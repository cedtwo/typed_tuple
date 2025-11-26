//! Submodule defining the `TupleKey` trait for associating marker types with
//! tuple indices.

use crate::prelude::*;

/// Helper trait to associate a marker type with a tuple index.
///
/// This trait can be used to define blanket implementations that work with
/// `TypedTuple<Idx, T>`. The marker can be either the type `T` itself or
/// a custom struct marker, allowing for flexible trait designs.
///
/// # Type Parameters
///
/// * `Marker` - A marker type used to identify which element to access. This
///   can be the actual element type `T` or some other known type.
///
/// # Examples
///
/// ```rust
/// use typed_tuple::prelude::{TupleIndex0, TupleIndex1, TupleIndex2, TupleKey, TypedTuple};
///
/// struct AgeMarker;
///
/// trait GetAge {
///     fn age(&self) -> u8;
/// }
///
/// impl<T> GetAge for T
/// where
///     Self: TypedTuple<<AgeMarker as TupleKey<Self>>::Idx, u8>,
///     AgeMarker: TupleKey<Self>,
/// {
///     fn age(&self) -> u8 {
///         *self.get()
///     }
/// }
///
/// impl TupleKey<(u8, f64, &str)> for AgeMarker {
///     type Idx = TupleIndex0;
/// }
///
/// impl TupleKey<(u8, &str, f64)> for AgeMarker {
///     type Idx = TupleIndex0;
/// }
///
/// impl TupleKey<(&str, f64, u8, bool)> for AgeMarker {
///     type Idx = TupleIndex2;
/// }
///
/// impl TupleKey<(&str, u8, f64)> for AgeMarker {
///     type Idx = TupleIndex1;
/// }
///
/// assert_eq!((67u8, "Alice", 3.5f64).age(), 67u8);
/// assert_eq!((15u8, 3.5f64, "Bob").age(), 15u8);
/// assert_eq!(("Charlie", 56u8, 3.5f64).age(), 56u8);
/// assert_eq!(("Diana", 4.2f64, 29u8, true).age(), 29u8);
/// ```
pub trait TupleKey<Marker> {
    /// The index of the element associated with the marker type.
    type Idx: TupleIndex;
}
