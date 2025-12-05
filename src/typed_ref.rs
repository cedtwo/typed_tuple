use typed_tuple_macros::impl_typed_ref;

/// # TypedRef
///
/// [`TypedRef`] creates a new tuple containing references of all inner elements
/// of `Self`. This is useful for for preserving the source tuple isolating
/// element(s). [`TypedRef`] is not necessary for small `!Copy` types. See also
/// [`TypedMut`].
pub trait TypedRef<'a>: Sized {
    /// The output tuple.
    type Output;

    /// Return a new tuple where all inner elements are borrowed.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use typed_tuple::{TypedRef, TypedSplit};
    /// let tuple = (String::from("a"), String::from("b"), String::from("c"));
    /// let tuple_ref = tuple.into_ref();
    ///
    /// let (_, last): (_, (_,)) = tuple_ref.split();
    ///
    /// assert_eq!(last.0, "c");
    /// assert_eq!(tuple, (String::from("a"), String::from("b"), String::from("c")));
    /// ```
    fn into_ref(&'a self) -> Self::Output;
}

/// # TypedRefMut
///
/// [`TypedMut`] creates a new tuple containing mutable references of all inner
/// elements of `Self`. This is useful for for preserving the source tuple when
/// isolating element(s). [`TypedMut`] is not necessary for small `!Copy` types.
/// See also [`TypedRef`].
pub trait TypedMut<'a>: Sized {
    /// The output tuple.
    type Output;

    /// Return a new tuple where all inner elements are mutably borrowed.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use typed_tuple::{TypedMut, TypedSplit};
    /// let mut tuple = (String::from("a"), String::from("b"), String::from("c"));
    /// let tuple_mut = tuple.into_mut();
    ///
    /// let (_, last): (_, (_,)) = tuple_mut.split();
    /// last.0.push('d');
    ///
    /// assert_eq!(last.0, "cd");
    /// assert_eq!(tuple, (String::from("a"), String::from("b"), String::from("cd")));
    /// ```
    fn into_mut(&'a mut self) -> Self::Output;
}

impl_typed_ref!(12);
