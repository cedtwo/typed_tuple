use typed_tuple_macros::impl_typed_extract;

/// Extract tuple patterns.
pub trait TypedExtract<const INDEX: usize, T>: Sized {
    /// Extract a part of the tuple.
    ///
    /// Tuples are extracted by specifying either a unique type pattern, or a type
    /// pattern with the index of the first element. Splitting a tuple may simplify
    /// isolating non-unique patterns. See [`TypedBound::split`](crate::typed_bound::TypedBound).
    ///
    /// # Example
    /// ```
    /// # use typed_tuple::TypedExtract;
    /// // Get by type.
    /// let tuple = (0u8, 1u16, 2u32, 3u64, 4u128);
    /// let extracted: (u16, _, _) = tuple.extract(); // Get 3 elements starting at the u16 field.
    /// assert_eq!(extracted, (1, 2, 3));
    ///
    /// // Get by type. If a pattern is not unique, an index must be specified.
    /// let tuple = (0u8, 1u16, 2u32, 3u8, 4u16, 5u32);
    /// let extracted: (u16, _) = TypedExtract::<4, _>::extract(tuple); // Get 3 elements starting at index 3.
    /// assert_eq!(extracted, (4, 5));
    /// ```
    fn extract(self) -> T;
}

impl_typed_extract!(10);
