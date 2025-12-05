use typed_tuple_macros::impl_typed_index;

/// ## TypedIndex
///
/// [`TypedIndex`] enables tuple element access by inferring the *unique* element,
/// type, or passing an explicit numeric index.
///
/// [`TypedIndex::get`] consumes `Self` returning an element, borrows `&Self`, returning
/// an element reference, and mutably borrows `&mut Self` returning a mutable element
/// reference.
///
/// ```rust
/// # use typed_tuple::TypedIndex;
/// let mut tuple = (0u8, 1u16, 2u32);
/// let el_ref: &u16 = (&tuple).get();
/// let el_mut: &mut u16 = (&mut tuple).get();
/// let el: u16 = tuple.get();
/// ```
///
/// ## Get by type
///
/// An index is inferred by either specifying, or inferring a *unique* element type.
///
/// ```rust
/// # use typed_tuple::TypedIndex;
/// let tuple = (0u8, 1u16, 2u32, 3u64, 4u128);
/// let element: u64 = tuple.get(); // Get the unique `u64` element.
/// assert_eq!(element, 3);
/// ```
///
/// ## Get by index
///
/// Where a unique type cannot be specified, the element `INDEX` must be specified.
///
/// ```rust
/// # use typed_tuple::TypedIndex;
/// let tuple = (0u8, 1u16, 2u32, 3u64, 4u128);
/// let element = TypedIndex::<3, _>::get(tuple); // Split at index 3.
/// assert_eq!(element, 3);
/// ```
pub trait TypedIndex<const INDEX: usize, T> {
    /// Get a reference to the element of type `T`.
    ///
    /// # Example
    /// ```
    /// # use typed_tuple::TypedIndex;
    /// let tuple = ("a", 'b', 2usize);
    ///
    /// // Get by type.
    /// let element: &str = tuple.get();
    /// assert_eq!(element, "a");
    ///
    /// // Get by index.
    /// let element = TypedIndex::<1, _>::get(tuple);
    /// assert_eq!(element, 'b');
    /// ```
    fn get(self) -> T;
}

impl_typed_index!(12);
