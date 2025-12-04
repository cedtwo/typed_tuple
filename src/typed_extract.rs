use typed_tuple_macros::impl_typed_extract;

/// ## TypedExtract
///
/// [`TypedExtract`] allows for tuple patterns to be extracted from a given tuple.
/// A tuple range can either be specified by inferring a tuple sequence for extraction,
/// or by explicitly defining an inclusive and exclusive lower and upper bound respectively.
///
/// ## Type pattern extraction
///
/// An inferred **unique** tuple pattern can be extracted without providing an explicit
/// index/indices. For non-unique tuple patterns, providing either (or both) the `INDEX_START`
/// or `INDEX_END` argument is required.
///
/// ```rust
/// # use typed_tuple::TypedExtract;
/// let tuple = (0u8, 1u16, 2u32, 3u64, 4u128, 5u8, 6u16, 7u32);
///
/// // Get the 3 elements starting from type `u16`.
/// let extracted: (u16, _, _) = tuple.extract();
/// assert_eq!(extracted, (1, 2, 3));
///
/// // Get the 3 elements ending at type `u128`.
/// let extracted: (_, _, u128) = tuple.extract();
/// assert_eq!(extracted, (2, 3, 4));
///
/// // Get 3 elements starting from type `u16`. Specify index `8` as the (exclusive) end index.
/// // Either (or both) a lower or upper bound must be specified as `(u16, _)` is not a unique pattern.
/// let extracted: (u16, _,) = TypedExtract::<_, 8, _>::extract(tuple);
/// assert_eq!(extracted, (6, 7));
/// ```
///
/// ## Index (range) extraction
///
/// Where a tuple pattern is not inferred, both the `INDEX_START` and `INDEX_END` arguments
/// must be specified. Indices are representative of an inclusive and exclusive lower and upper
/// range bound respectively (analogous to the indices of [`std::ops::Range`]).
///
/// ```rust
/// # use typed_tuple::TypedExtract;
/// let tuple = (0u8, 1u16, 2u32, 3u64, 4u128, 5u8, 6u16, 7u32);
///
/// // Get elements of the index range `1..4`.
/// let extracted = TypedExtract::<1, 4, _>::extract(tuple);
/// assert_eq!(extracted, (1, 2, 3));
///
/// // Get elements of the index range `2..6`.
/// let extracted = TypedExtract::<2, 6, _>::extract(tuple);
/// assert_eq!(extracted, (2, 3, 4, 5));
///
/// ```
pub trait TypedExtract<const INDEX_START: usize, const INDEX_END: usize, T>: Sized {
    /// Extract sequentual elements inferred by a type pattern and/or indices.
    /// See the documentation of [`TypedExtract`] for usage.
    ///
    /// # Example
    /// ```
    /// # use typed_tuple::TypedExtract;
    /// let tuple = (0u8, 1u16, 2u32, 3u64, 4u128, 5u8, 6u16, 7u32);
    ///
    /// // Get the 3 elements starting from type `u16`.
    /// let extracted: (u16, _, _) = tuple.extract();
    /// assert_eq!(extracted, (1, 2, 3));
    ///
    /// // Get elements of the index range `2..6`.
    /// let extracted = TypedExtract::<2, 6, _>::extract(tuple);
    /// assert_eq!(extracted, (2, 3, 4, 5));
    /// ```
    fn extract(self) -> T;
}

impl_typed_extract!(12);
