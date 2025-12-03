use typed_tuple_macros::impl_typed_bound;

/// Trait for mapping elements of either side of an index (left inclusive).
pub trait TypedBound<const INDEX: usize, L, R>: Sized {
    /// Split the tuple.
    ///
    /// Tuples are split by providing either the leftmost elements `L`, the rightmost
    /// elements `R`, or the left-inclusive index `INDEX`.
    /// # Example
    /// ```
    /// # use typed_tuple::TypedBound;
    /// // Get by type.
    /// let tuple = (0u8, 1u16, 2u32, 3u64, 4u128);
    /// let (l, r): ((_, _, _), _) = tuple.split(); // Split the first three elements.
    /// assert!(l, (0, 1, 2));
    ///
    /// // Get by index (left inclusive).
    /// let tuple = (0u8, 1u16, 2u32, 3u64, 4u128);
    /// let (l, r) = TypedBound::<2, _, _>::split(tuple); // Split at index 2 (inclusive).
    /// assert!(r, (3, 4));
    /// ```
    fn split(self) -> (L, R);
}

impl_typed_bound!(12);
