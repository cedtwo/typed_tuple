#![doc = include_str!("../README.md")]
#![no_std]
#![cfg_attr(feature = "len_128", recursion_limit = "256")]

/// Helper trait to associate a marker type with a tuple index.
///
/// This trait can be used to define blanket implementations that work with
/// `TypedTuple<Idx, T>`. The marker can be either the type `T` itself or
/// a custom struct marker, allowing for flexible trait designs.
///
/// # Type Parameters
///
/// * `Marker` - A marker type used to identify which element to access. This
///   can be the actual element type `T` or some other known type.
///
/// # Examples
///
/// ```rust
/// use typed_tuple::{TupleIndex0, TupleIndex1, TupleIndex2, TupleKey, TypedTuple};
///
/// struct AgeMarker;
///
/// trait GetAge {
///     fn age(&self) -> u8;
/// }
///
/// impl<T> GetAge for T
/// where
///     Self: TypedTuple<<AgeMarker as TupleKey<Self>>::Idx, u8>,
///     AgeMarker: TupleKey<Self>,
/// {
///     fn age(&self) -> u8 {
///         *self.get()
///     }
/// }
///
/// impl TupleKey<(u8, f64, &str)> for AgeMarker {
///     type Idx = TupleIndex0;
/// }
///
/// impl TupleKey<(u8, &str, f64)> for AgeMarker {
///     type Idx = TupleIndex0;
/// }
///
/// impl TupleKey<(&str, f64, u8, bool)> for AgeMarker {
///     type Idx = TupleIndex2;
/// }
///
/// impl TupleKey<(&str, u8, f64)> for AgeMarker {
///     type Idx = TupleIndex1;
/// }
///
/// assert_eq!((67u8, "Alice", 3.5f64).age(), 67u8);
/// assert_eq!((15u8, 3.5f64, "Bob").age(), 15u8);
/// assert_eq!(("Charlie", 56u8, 3.5f64).age(), 56u8);
/// assert_eq!(("Diana", 4.2f64, 29u8, true).age(), 29u8);
/// ```
pub trait TupleKey<Marker> {
    /// The index of the element associated with the marker type.
    type Idx;
}

/// Trait for tuple element manipulation by type.
pub trait TypedTuple<Idx, T> {
    /// The type of the remaining tuple after popping element of type `T`.
    type PopOutput;
    /// The type of the left tuple when splitting at [.., INDEX].
    type SplitLeft: TypedTuple<Idx, T>;
    /// The type of the right tuple when splitting at (INDEX, ..].
    type SplitRight;
    /// The associated index.
    const INDEX: usize;

    /// Get a reference to the element of type `T`.
    /// # Example
    /// ```
    /// # use typed_tuple::*;
    /// // Get by type.
    /// let tuple = ("a", 'b', 2usize);
    /// let a: &&str = tuple.get();
    /// let b: &char = tuple.get();
    /// let c: &usize = tuple.get();
    ///
    /// // Get by 'const' index.
    /// let a = TypedTuple::<TupleIndex0, _>::get(&tuple);
    /// let b = TypedTuple::<TupleIndex1, _>::get(&tuple);
    /// let c = TypedTuple::<TupleIndex2, _>::get(&tuple);
    /// ```
    fn get(&self) -> &T;

    /// Get a mutable reference to the element of type `T`.
    /// # Example
    /// ```
    /// # use typed_tuple::*;
    /// // Mutate by type.
    /// let mut tuple = ("a", 'b', 2usize);
    /// *tuple.get_mut() = "c";
    /// *tuple.get_mut() = 'd';
    /// *tuple.get_mut() = 3usize;
    /// assert_eq!(tuple, ("c", 'd', 3));
    ///
    /// // Mutate by 'const' index.
    /// *TypedTuple::<TupleIndex0, _>::get_mut(&mut tuple) = "e";
    /// *TypedTuple::<TupleIndex1, _>::get_mut(&mut tuple) = 'f';
    /// *TypedTuple::<TupleIndex2, _>::get_mut(&mut tuple) = 4usize;
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
    /// # use typed_tuple::*;
    /// // Map by type.
    /// let mut tuple = ("a".to_string(), 1u8, 2usize);
    /// tuple.map(|el: String| el.to_uppercase());
    /// tuple.map(|el: u8| el + 1);
    /// tuple.map(|el: usize| el + 2);
    /// assert_eq!(tuple, ("A".to_string(), 2, 4));
    ///
    /// // Map by 'const' index.
    /// TypedTuple::<TupleIndex0, _>::map(&mut tuple, |el| el.to_lowercase());
    /// TypedTuple::<TupleIndex1, _>::map(&mut tuple, |el| el - 1);
    /// TypedTuple::<TupleIndex2, _>::map(&mut tuple, |el| el - 2);
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
    /// # use typed_tuple::*;
    /// // Pop by type.
    /// let tuple = ("a", 'b', 2usize);
    /// let (s, rest): (&str, _) = tuple.pop();
    /// assert_eq!(s, "a");
    /// assert_eq!(rest, ('b', 2usize));
    ///
    /// // Pop by 'const' index.
    /// let tuple = ("a", 'b', 2usize);
    /// let (c, rest) = TypedTuple::<TupleIndex1, _>::pop(tuple);
    /// assert_eq!(c, 'b');
    /// assert_eq!(rest, ("a", 2usize));
    /// ```
    fn pop(self) -> (T, Self::PopOutput);

    #[inline]
    /// Swaps the element at INDEX with the element at OTHER_INDEX.
    ///
    /// Both indices must contain elements of type `T`. If INDEX == OTHER_INDEX,
    /// this is a no-op.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use typed_tuple::*;
    /// let mut tuple = (1u32, "hello", 2u32, 'x', 3u32);
    /// TypedTuple::<TupleIndex0, u32>::swap::<TupleIndex2>(&mut tuple);
    /// assert_eq!(tuple, (2u32, "hello", 1u32, 'x', 3u32));
    /// ```
    fn swap<Other>(&mut self)
    where
        Self: TypedTuple<Other, T>,
    {
        if <Self as TypedTuple<Idx, T>>::INDEX != <Self as TypedTuple<Other, T>>::INDEX {
            unsafe {
                let ptr = self as *mut Self;
                let field1 = <Self as TypedTuple<Idx, T>>::get_mut(&mut *ptr);
                let field2 = <Self as TypedTuple<Other, T>>::get_mut(&mut *ptr);
                core::mem::swap(field1, field2);
            }
        }
    }

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
    /// the remaining elements. The left tuple has length
    /// INDEX + 1, and the right tuple has length total_length - (INDEX + 1).
    /// The left tuple will `[.., INDEX]` and the right tuple will be
    /// `(INDEX, ..]`, or analogously, `[INDEX + 1, ..]`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use typed_tuple::*;
    /// let tuple = (1u8, 2u16, 3u32, 4u64, 5i8);
    /// let (left, right) = TypedTuple::<TupleIndex2, u32>::split_at(tuple);
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
