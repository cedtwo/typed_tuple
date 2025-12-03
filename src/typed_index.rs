use typed_tuple_macros::impl_typed_index;

/// Trait for tuple element manipulation by type.
pub trait TypedIndex<const INDEX: usize, T> {
    /// Get a reference to the element of type `T`.
    /// # Example
    /// ```
    /// # use typed_tuple::TypedIndex;
    /// // Get by type.
    /// let tuple = ("a", 'b', 2usize);
    /// let a: &&str = tuple.get();
    /// let b: &char = tuple.get();
    /// let c: &usize = tuple.get();
    ///
    /// // Get by 'const' index.
    /// let a = TypedIndex::<0, _>::get(&tuple);
    /// let b = TypedIndex::<1, _>::get(&tuple);
    /// let c = TypedIndex::<2, _>::get(&tuple);
    /// ```
    fn get(&self) -> &T;

    /// Get a mutable reference to the element of type `T`.
    /// # Example
    /// ```
    /// # use typed_tuple::TypedIndex;
    /// // Mutate by type.
    /// let mut tuple = ("a", 'b', 2usize);
    /// *tuple.get_mut() = "c";
    /// *tuple.get_mut() = 'd';
    /// *tuple.get_mut() = 3usize;
    /// assert_eq!(tuple, ("c", 'd', 3));
    ///
    /// // Mutate by 'const' index.
    /// *TypedIndex::<0, _>::get_mut(&mut tuple) = "e";
    /// *TypedIndex::<1, _>::get_mut(&mut tuple) = 'f';
    /// *TypedIndex::<2, _>::get_mut(&mut tuple) = 4usize;
    /// assert_eq!(tuple, ("e", 'f', 4))
    /// ```
    fn get_mut(&mut self) -> &mut T;
}

impl_typed_index!(12);
