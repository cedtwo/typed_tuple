//! Arithmetic operations for tuple indices.

use crate::prelude::*;

/// Trait for adding two tuple indices.
///
/// This trait allows compile-time addition of tuple index positions.
/// It's only implemented for index combinations that result in a valid index
/// (i.e., the sum must be less than the maximum tuple size).
///
/// # Examples
///
/// ```rust
/// # use typed_tuple::prelude::*;
/// // TupleIndex2 + TupleIndex3 = TupleIndex5
/// type Result = <TupleIndex2 as TupleIndexAdd<TupleIndex3>>::Output;
/// assert_eq!(<Result as TupleIndex>::INDEX, 5);
///
/// // TupleIndex0 + TupleIndex10 = TupleIndex10
/// type Result2 = <TupleIndex0 as TupleIndexAdd<TupleIndex10>>::Output;
/// assert_eq!(<Result2 as TupleIndex>::INDEX, 10);
/// ```
pub trait TupleIndexAdd<Other> {
    /// The resulting tuple index type after addition.
    type Output: TupleIndex;
}

/// Trait for subtracting two tuple indices.
///
/// This trait allows compile-time subtraction of tuple index positions.
/// It's only implemented for index combinations where the first index is
/// greater than or equal to the second (i.e., no negative results).
///
/// # Examples
///
/// ```rust
/// # use typed_tuple::prelude::*;
/// // TupleIndex5 - TupleIndex2 = TupleIndex3
/// type Result = <TupleIndex5 as TupleIndexSub<TupleIndex2>>::Output;
/// assert_eq!(<Result as TupleIndex>::INDEX, 3);
///
/// // TupleIndex10 - TupleIndex10 = TupleIndex0
/// type Result2 = <TupleIndex10 as TupleIndexSub<TupleIndex10>>::Output;
/// assert_eq!(<Result2 as TupleIndex>::INDEX, 0);
/// ```
pub trait TupleIndexSub<Other> {
    /// The resulting tuple index type after subtraction.
    type Output: TupleIndex;
}

/// Trait for subtracting two tuple indices with saturation.
///
/// This trait allows compile-time subtraction of tuple index positions.
/// Unlike `TupleIndexSub`, this trait is implemented for all index
/// combinations. When the subtraction would result in a negative value, the
/// result saturates at TupleIndex0.
///
/// # Examples
///
/// ```rust
/// # use typed_tuple::prelude::*;
/// // TupleIndex5 - TupleIndex2 = TupleIndex3
/// type Result = <TupleIndex5 as TupleIndexSaturatingSub<TupleIndex2>>::Output;
/// assert_eq!(<Result as TupleIndex>::INDEX, 3);
///
/// // TupleIndex2 - TupleIndex10 = TupleIndex0 (saturates at 0)
/// type Result2 = <TupleIndex2 as TupleIndexSaturatingSub<TupleIndex10>>::Output;
/// assert_eq!(<Result2 as TupleIndex>::INDEX, 0);
/// ```
pub trait TupleIndexSaturatingSub<Other> {
    /// The resulting tuple index type after saturating subtraction.
    type Output: TupleIndex;
}
