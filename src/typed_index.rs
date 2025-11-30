//! Sub-module for type/index mapping.

use crate::prelude::*;

/// Trait for mapping an index to a type.
pub trait TypedIndex<Idx: TupleIndex, T> {
    /// Get a reference to the element of type `T`.
    /// # Example
    /// ```
    /// # use typed_tuple::prelude::*;
    /// // Get by type.
    /// let tuple = ("a", 'b', 2usize);
    /// let a: &&str = tuple.get_at();
    /// let b: &char = tuple.get_at();
    /// let c: &usize = tuple.get_at();
    ///
    /// // Get by 'const' index.
    /// let a = TypedIndex::<TupleIndex0, _>::get_at(&tuple);
    /// let b = TypedIndex::<TupleIndex1, _>::get_at(&tuple);
    /// let c = TypedIndex::<TupleIndex2, _>::get_at(&tuple);
    /// ```
    fn get_at(&self) -> &T;

    /// Get a mutable reference to the element of type `T`.
    /// # Example
    /// ```
    /// # use typed_tuple::prelude::*;
    /// // Mutate by type.
    /// let mut tuple = ("a", 'b', 2usize);
    /// *tuple.get_mut_at() = "c";
    /// *tuple.get_mut_at() = 'd';
    /// *tuple.get_mut_at() = 3usize;
    /// assert_eq!(tuple, ("c", 'd', 3));
    ///
    /// // Mutate by 'const' index.
    /// *TypedIndex::<TupleIndex0, _>::get_mut_at(&mut tuple) = "e";
    /// *TypedIndex::<TupleIndex1, _>::get_mut_at(&mut tuple) = 'f';
    /// *TypedIndex::<TupleIndex2, _>::get_mut_at(&mut tuple) = 4usize;
    /// assert_eq!(tuple, ("e", 'f', 4))
    /// ```
    fn get_mut_at(&mut self) -> &mut T;
}
