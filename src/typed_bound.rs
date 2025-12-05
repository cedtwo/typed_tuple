use typed_tuple_macros::impl_typed_bound;

/// ## TypedBound
///
/// [`TypedBound`] allows for a tuple to be split given a type patterns. A tuple
/// can be split either specifying the leftmost or rightmost tuple pattern, or by
/// inferring a tuple sequence for extraction, or by explicitly defining the
/// left-exclusive/right-inclusive center bound index `INDEX`.
///
/// ## Type pattern splitting
///
/// An index will be inferred by, at a minimum, specifying the number of elements
/// in either (or both) the left or right segment.
///
/// ```rust
/// # use typed_tuple::TypedBound;
/// let tuple = (0u8, 1u16, 2u32, 3u64, 4u128);
///
/// // Split after the first 3 elements.
/// let (left, right): ((_, _, _), _) = tuple.split();
/// assert_eq!(left, (0, 1, 2));
/// assert_eq!(right, (3, 4));
///
/// // Split prior to the last element.
/// let (left, right): (_, (_,)) = tuple.split();
/// assert_eq!(left, (0, 1, 2, 3));
/// assert_eq!(right, (4,));
/// ```
///
/// ## Index splitting
///
/// Where a tuple pattern is not inferred, the center bound index `INDEX` must be
/// specified. The given index is representative of the exclusive upper bound of
/// the left segment, and the inclusive lower bound of the right segment (analogous
/// to the indices of split operations in [`core::slice`]).
///
/// ```rust
/// # use typed_tuple::TypedBound;
/// let tuple = (0u8, 1u16, 2u32, 3u64, 4u128);
///
/// // Split at index 3.
/// let (left, right) = TypedBound::<3, _, _>::split(tuple);
/// assert_eq!(left, (0, 1, 2));
/// assert_eq!(right, (3, 4));
///
/// // Split at index 0.
/// let (left, right) = TypedBound::<0, _, _>::split(tuple);
/// assert_eq!(left, ());
/// assert_eq!(right, (0, 1, 2, 3, 4));
/// ```
pub trait TypedBound<const INDEX: usize, L, R>: Sized {
    /// Split a tuple by specifying the leftmost elements, rightmost elements, or
    /// the (left-exclusive/right-inclusive) center bound index `INDEX`. See the
    /// documentation of [`TypedBound`] for usage.
    ///
    /// # Example
    /// ```
    /// # use typed_tuple::TypedBound;
    /// let tuple = (0u8, 1u16, 2u32, 3u64, 4u128);
    ///
    /// // Infer an index given the left segment.
    /// let (l, r): ((_, _, _), _) = tuple.split();
    /// assert_eq!(l, (0, 1, 2));
    /// assert_eq!(r, (3, 4));
    ///
    /// // Specify an index (2) for splitting the left/right segment.
    /// let (l, r) = TypedBound::<2, _, _>::split(tuple);
    /// assert_eq!(l, (0, 1));
    /// assert_eq!(r, (2, 3, 4));
    /// ```
    fn split(self) -> (L, R);
}

impl_typed_bound!(12);
