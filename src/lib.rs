#![doc = include_str!("../README.md")]
#![no_std]
#![cfg_attr(feature = "len_128", recursion_limit = "256")]

/// Trait for tuple element manipulation by type.
pub trait TypedTuple<const INDEX: usize, T> {
    /// The type of the remaining tuple after popping element of type `T`.
    type PopOutput;
    /// The type of the left tuple when splitting at INDEX (inclusive).
    type SplitLeft;
    /// The type of the right tuple when splitting at INDEX (inclusive).
    type SplitRight;

    /// Get a reference to the element of type `T`.
    /// # Example
    /// ```
    /// # use typed_tuple::TypedTuple;
    /// // Get by type.
    /// let tuple = ("a", 'b', 2usize);
    /// let a: &&str = tuple.get();
    /// let b: &char = tuple.get();
    /// let c: &usize = tuple.get();
    ///
    /// // Get by 'const' index.
    /// let a = TypedTuple::<0, _>::get(&tuple);
    /// let b = TypedTuple::<1, _>::get(&tuple);
    /// let c = TypedTuple::<2, _>::get(&tuple);
    /// ```
    fn get(&self) -> &T;

    /// Get a mutable reference to the element of type `T`.
    /// # Example
    /// ```
    /// # use typed_tuple::TypedTuple;
    /// // Mutate by type.
    /// let mut tuple = ("a", 'b', 2usize);
    /// *tuple.get_mut() = "c";
    /// *tuple.get_mut() = 'd';
    /// *tuple.get_mut() = 3usize;
    /// assert_eq!(tuple, ("c", 'd', 3));
    ///
    /// // Mutate by 'const' index.
    /// *TypedTuple::<0, _>::get_mut(&mut tuple) = "e";
    /// *TypedTuple::<1, _>::get_mut(&mut tuple) = 'f';
    /// *TypedTuple::<2, _>::get_mut(&mut tuple) = 4usize;
    /// assert_eq!(tuple, ("e", 'f', 4))
    /// ```
    fn get_mut(&mut self) -> &mut T;

    /// Replaces the element of type `T` with the provided value, returning the
    /// old value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to replace the element with.
    ///
    /// # Returns
    ///
    /// The old value that was replaced.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use typed_tuple::TypedTuple;
    /// let mut tuple = (10u32, 20u64);
    /// let old = tuple.replace(30u32);
    /// assert_eq!(old, 10u32);
    /// assert_eq!(tuple, (30u32, 20u64));
    /// ```
    #[inline]
    fn replace(&mut self, value: T) -> T {
        core::mem::replace(self.get_mut(), value)
    }

    /// Takes a closure, mutating the element of type `T`.
    /// # Example
    /// ```
    /// # use typed_tuple::TypedTuple;
    /// // Map by type.
    /// let mut tuple = ("a".to_string(), 1u8, 2usize);
    /// tuple.map(|el: String| el.to_uppercase());
    /// tuple.map(|el: u8| el + 1);
    /// tuple.map(|el: usize| el + 2);
    /// assert_eq!(tuple, ("A".to_string(), 2, 4));
    ///
    /// // Map by 'const' index.
    /// TypedTuple::<0, _>::map(&mut tuple, |el| el.to_lowercase());
    /// TypedTuple::<1, _>::map(&mut tuple, |el| el - 1);
    /// TypedTuple::<2, _>::map(&mut tuple, |el| el - 2);
    /// assert_eq!(tuple, ("a".to_string(), 1, 2))
    /// ```
    fn map<FN: FnOnce(T) -> T>(&mut self, f: FN)
    where
        T: Default;

    /// Removes and returns the element of type `T` from the tuple, along with
    /// the remaining tuple.
    ///
    /// # Returns
    ///
    /// A tuple containing the removed element and the remaining tuple with all
    /// other elements.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use typed_tuple::TypedTuple;
    /// // Pop by type.
    /// let tuple = ("a", 'b', 2usize);
    /// let (s, rest): (&str, _) = tuple.pop();
    /// assert_eq!(s, "a");
    /// assert_eq!(rest, ('b', 2usize));
    ///
    /// // Pop by 'const' index.
    /// let tuple = ("a", 'b', 2usize);
    /// let (c, rest) = TypedTuple::<1, _>::pop(tuple);
    /// assert_eq!(c, 'b');
    /// assert_eq!(rest, ("a", 2usize));
    /// ```
    fn pop(self) -> (T, Self::PopOutput);

    /// Swaps the element at INDEX with the element at OTHER_INDEX.
    ///
    /// Both indices must contain elements of type `T`. If INDEX == OTHER_INDEX,
    /// this is a no-op.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use typed_tuple::TypedTuple;
    /// let mut tuple = (1u32, "hello", 2u32, 'x', 3u32);
    /// TypedTuple::<0, u32>::swap::<2>(&mut tuple);
    /// assert_eq!(tuple, (2u32, "hello", 1u32, 'x', 3u32));
    /// ```
    fn swap<const OTHER_INDEX: usize>(&mut self)
    where
        Self: TypedTuple<OTHER_INDEX, T>;

    /// Takes the element of type `T`, replacing it with the default value.
    ///
    /// # Returns
    ///
    /// The old value of the element.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use typed_tuple::TypedTuple;
    /// let mut tuple = (String::from("hello"), 42, 3.14);
    /// let s: String = tuple.take();
    /// assert_eq!(s, "hello");
    /// assert_eq!(tuple, (String::new(), 42, 3.14));
    /// ```
    #[inline]
    fn take(&mut self) -> T
    where
        T: Default,
    {
        core::mem::take(self.get_mut())
    }

    /// Splits the tuple at INDEX (inclusive), returning two tuples.
    ///
    /// The element at INDEX is included in the left tuple.
    ///
    /// # Returns
    ///
    /// A tuple containing (left_tuple, right_tuple) where left_tuple contains
    /// elements from index 0 to INDEX (inclusive), and right_tuple contains
    /// the remaining elements.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use typed_tuple::TypedTuple;
    /// let tuple = (1u8, 2u16, 3u32, 4u64, 5i8);
    /// let (left, right) = TypedTuple::<2, u32>::split_at(tuple);
    /// assert_eq!(left, (1u8, 2u16, 3u32));
    /// assert_eq!(right, (4u64, 5i8));
    /// ```
    fn split_at(self) -> (Self::SplitLeft, Self::SplitRight);
}

// Generate all tuple implementations using the proc macro
#[cfg(not(feature = "len_128"))]
typed_tuple_macros::generate_typed_tuple_impls!(64);

#[cfg(feature = "len_128")]
typed_tuple_macros::generate_typed_tuple_impls!(128);
