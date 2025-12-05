use typed_tuple_macros::impl_typed_extract;

/// ## TypedExtract
///
/// [`TypedExtract`] allows for tuple patterns to be extracted from a given tuple.
/// A tuple range can either be specified by inferring a tuple sequence for extraction,
/// or by explicitly defining an inclusive and exclusive lower and upper bound respectively.
///
/// [`TypedExtract::extract`] consumes `Self` returning the specified elements, borrows `&Self`,
/// returning element references, and mutably borrow `&mut Self` returning mutable element
/// references.
///
/// ```rust
/// # use typed_tuple::TypedExtract;
/// let mut tuple = (0u8, 1u16, 2u32);
/// let element_ref: (&u16,) = (&tuple).extract();
/// let element_mut: (&mut u16,) = (&mut tuple).extract();
/// let element: (u16,) = tuple.extract();
/// ```
///
/// ## Extract by type
///
/// An inferred **unique** tuple pattern can be extracted without providing an explicit
/// index/indices. For non-unique tuple patterns, providing either (or both) the `INDEX_START`
/// or `INDEX_END` argument is required.
///
/// ```rust
/// # use typed_tuple::TypedExtract;
/// let tuple = (0u8, 1u16, 2u32, 3u64, 4u128, 5u8, 6u16, 7u32);
/// let extracted: (u16, _, _) = tuple.extract(); // Get the 3 elements starting from type `u16`.
/// assert_eq!(extracted, (1, 2, 3));
/// let extracted: (_, _, u128) = tuple.extract(); // Get the 3 elements ending at type `u128`.
/// assert_eq!(extracted, (2, 3, 4));
///
/// ```
///
/// ## Extract by index (range)
///
/// Where a tuple pattern is not inferred, both the `INDEX_START` and `INDEX_END` arguments
/// must be specified. Indices are representative of an inclusive and exclusive lower and upper
/// range bound respectively (analogous to the indices of [`core::ops::Range`]).
///
/// ```rust
/// # use typed_tuple::TypedExtract;
/// let tuple = (0u8, 1u16, 2u32, 3u64, 4u128, 5u8, 6u16, 7u32);
/// let extracted = TypedExtract::<1, 4, _>::extract(tuple); // Get elements of the index range `1..4`.
/// assert_eq!(extracted, (1, 2, 3));
/// let extracted = TypedExtract::<2, 6, _>::extract(tuple); // Get elements of the index range `2..6`.
/// assert_eq!(extracted, (2, 3, 4, 5));
/// let extracted: (_, _,) = TypedExtract::<_, 8, _>::extract(tuple); // Get the 2 elements ending at the (exclusive) index 8.
/// assert_eq!(extracted, (6, 7));
/// ```
pub trait TypedExtract<const INDEX_START: usize, const INDEX_END: usize, T>: Sized {
    /// Extract sequentual elements inferred by a type pattern and/or indices.
    /// See [`TypedExtract`] for extended documentation.
    ///
    /// # Example
    /// ```
    /// # use typed_tuple::TypedExtract;
    /// let tuple = (0u8, 1u16, 2u32, 3u64, 4u128, 5u8, 6u16, 7u32);
    ///
    /// // Infer an element segment.
    /// let extracted: (u16, _, _) = tuple.extract();
    /// assert_eq!(extracted, (1, 2, 3));
    /// // Specify an element segment by index range (2..6).
    /// let extracted = TypedExtract::<2, 6, _>::extract(tuple);
    /// assert_eq!(extracted, (2, 3, 4, 5));
    /// ```
    fn extract(self) -> T;
}

impl_typed_extract!(12);
