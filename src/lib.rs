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

/// Trait for tuple index types.
pub trait TupleIndex {
    /// The associated index value.
    const INDEX: usize;
}

/// Trait for tuple element manipulation by type.
pub trait TypedTuple<Idx, T> {
    /// The type of the remaining tuple after popping element of type `T`.
    type PopOutput;
    /// The type of the left tuple when splitting exclusively (excludes element
    /// at INDEX): [.., INDEX).
    type SplitLeftExclusive;
    /// The type of the left tuple when splitting inclusively (includes element
    /// at INDEX): [.., INDEX].
    type SplitLeftInclusive: TypedTuple<Idx, T>;
    /// The type of the right tuple when splitting exclusively (excludes element
    /// at INDEX): (INDEX, ..].
    type SplitRightExclusive;
    /// The type of the right tuple when splitting inclusively (includes element
    /// at INDEX): [INDEX, ..].
    type SplitRightInclusive: TypedTuple<TupleIndex0, T>;

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
    /// // Apply by type.
    /// let mut tuple = ("a".to_string(), 1u8, 2usize);
    /// tuple.apply(|el: &mut String| *el = el.to_uppercase());
    /// tuple.apply(|el: &mut u8| *el += 1);
    /// tuple.apply(|el: &mut usize| *el += 2);
    /// assert_eq!(tuple, ("A".to_string(), 2, 4));
    ///
    /// // Apply by 'const' index.
    /// TypedTuple::<TupleIndex0, _>::apply(&mut tuple, |el| *el = el.to_lowercase());
    /// TypedTuple::<TupleIndex1, _>::apply(&mut tuple, |el| *el -= 1);
    /// TypedTuple::<TupleIndex2, _>::apply(&mut tuple, |el| *el -= 2);
    /// assert_eq!(tuple, ("a".to_string(), 1, 2))
    /// ```
    fn apply<FN: FnOnce(&mut T)>(&mut self, f: FN) {
        f(self.get_mut());
    }

    /// Returns the outcome of the provided closure when applied to the element
    /// of type `T`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use typed_tuple::*;
    ///
    /// let tuple = (10u32, 20u64);
    /// let result = tuple.map(|x: &u32| x * 2);
    /// assert_eq!(result, 20u32);
    ///
    /// let result = tuple.map(|x: &u64| x + 5);
    /// assert_eq!(result, 25u64);
    /// ```
    fn map<FN, R>(&self, f: FN) -> R
    where
        FN: FnOnce(&T) -> R,
    {
        f(self.get())
    }

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
        Idx: TupleIndex,
        Other: TupleIndex,
    {
        if Idx::INDEX != Other::INDEX {
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

    /// Splits the tuple exclusively at INDEX, returning the element and the
    /// surrounding tuples.
    ///
    /// # Returns
    ///
    /// A tuple containing (left_exclusive, element, right_exclusive) where:
    /// - `left_exclusive` contains elements [0..INDEX)
    /// - `element` is the element at INDEX
    /// - `right_exclusive` contains elements (INDEX..)
    ///
    /// # Example
    ///
    /// ```rust
    /// # use typed_tuple::*;
    /// let tuple = (1u8, 2u16, 3u32, 4u64, 5i8);
    /// let (left, element, right) = TypedTuple::<TupleIndex2, u32>::split_exclusive(tuple);
    /// assert_eq!(left, (1u8, 2u16));
    /// assert_eq!(element, 3u32);
    /// assert_eq!(right, (4u64, 5i8));
    /// ```
    fn split_exclusive(self) -> (Self::SplitLeftExclusive, T, Self::SplitRightExclusive);

    /// Splits the tuple at INDEX (inclusive left), returning two tuples.
    ///
    /// The element at INDEX is included in the left tuple.
    ///
    /// # Returns
    ///
    /// A tuple containing (left_inclusive, right_exclusive) where:
    /// - `left_inclusive` contains elements [0..=INDEX]
    /// - `right_exclusive` contains elements (INDEX..)
    ///
    /// # Example
    ///
    /// ```rust
    /// # use typed_tuple::*;
    /// let tuple = (1u8, 2u16, 3u32, 4u64, 5i8);
    /// let (left, right) = TypedTuple::<TupleIndex2, u32>::split_left(tuple);
    /// assert_eq!(left, (1u8, 2u16, 3u32));
    /// assert_eq!(right, (4u64, 5i8));
    /// ```
    fn split_left(self) -> (Self::SplitLeftInclusive, Self::SplitRightExclusive);

    /// Splits the tuple at INDEX (inclusive right), returning two tuples.
    ///
    /// The element at INDEX is included in the right tuple.
    ///
    /// # Returns
    ///
    /// A tuple containing (left_exclusive, right_inclusive) where:
    /// - `left_exclusive` contains elements [0..INDEX)
    /// - `right_inclusive` contains elements [INDEX..]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use typed_tuple::*;
    /// let tuple = (1u8, 2u16, 3u32, 4u64, 5i8);
    /// let (left, right) = TypedTuple::<TupleIndex2, u32>::split_right(tuple);
    /// assert_eq!(left, (1u8, 2u16));
    /// assert_eq!(right, (3u32, 4u64, 5i8));
    /// ```
    fn split_right(self) -> (Self::SplitLeftExclusive, Self::SplitRightInclusive);

    /// Splits the tuple at INDEX (inclusive both), returning two tuples.
    ///
    /// The element at INDEX is included in both tuples (cloned).
    ///
    /// # Returns
    ///
    /// A tuple containing (left_inclusive, right_inclusive) where:
    /// - `left_inclusive` contains elements [0..=INDEX]
    /// - `right_inclusive` contains elements [INDEX..]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use typed_tuple::*;
    /// let tuple = (1u8, 2u16, 3u32, 4u64, 5i8);
    /// let (left, right) = TypedTuple::<TupleIndex2, u32>::split_inclusive(tuple);
    /// assert_eq!(left, (1u8, 2u16, 3u32));
    /// assert_eq!(right, (3u32, 4u64, 5i8));
    /// ```
    fn split_inclusive(self) -> (Self::SplitLeftInclusive, Self::SplitRightInclusive)
    where
        T: Clone;
}

/// Extension trait to add additional methods to TypedTuple.
pub trait TypedTupleExt<T>: Sized {
    #[inline]
    /// Pops the element of type `T` from the tuple, returning it along with
    /// the remaining tuple.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use typed_tuple::*;
    /// let mut tuple = (1u8, 2u16, 3u32);
    /// let (val, rest) = tuple.pop_at::<TupleIndex2>();
    /// assert_eq!(val, 3u32);
    /// assert_eq!(rest, (1u8, 2u16));
    /// ```
    fn pop_at<Idx>(self) -> (T, Self::PopOutput)
    where
        Self: TypedTuple<Idx, T>,
    {
        <Self as TypedTuple<Idx, T>>::pop(self)
    }

    #[inline]
    /// Swaps the element of type `T` at Idx with the element at Other.
    ///
    /// Both indices must contain elements of type `T`. If Idx == Other,
    /// this is a no-op.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use typed_tuple::*;
    /// let mut tuple = (1u32, "hello", 2u32, 'x', 3u32);
    /// tuple.swap_at::<TupleIndex0, TupleIndex2>();
    /// assert_eq!(tuple, (2u32, "hello", 1u32, 'x', 3u32));
    /// ```
    fn swap_at<Idx, Other>(&mut self)
    where
        Self: TypedTuple<Idx, T> + TypedTuple<Other, T>,
        Idx: TupleIndex,
        Other: TupleIndex,
    {
        <Self as TypedTuple<Idx, T>>::swap::<Other>(self);
    }

    #[inline]
    /// Splits the tuple at Idx (inclusive left), returning two tuples.
    ///
    /// The element at Idx is included in the left tuple.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use typed_tuple::*;
    /// let tuple = (1u8, 2u16, 3u32, 4u64, 5i8);
    /// let (left, right) = tuple.split_left_at::<TupleIndex2>();
    /// assert_eq!(left, (1u8, 2u16, 3u32));
    /// assert_eq!(right, (4u64, 5i8));
    /// ```
    fn split_left_at<Idx>(self) -> (Self::SplitLeftInclusive, Self::SplitRightExclusive)
    where
        Self: TypedTuple<Idx, T>,
    {
        <Self as TypedTuple<Idx, T>>::split_left(self)
    }

    #[inline]
    /// Splits the tuple at Idx (inclusive right), returning two tuples.
    ///
    /// The element at Idx is included in the right tuple.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use typed_tuple::*;
    /// let tuple = (1u8, 2u16, 3u32, 4u64, 5i8);
    /// let (left, right) = tuple.split_right_at::<TupleIndex2>();
    /// assert_eq!(left, (1u8, 2u16));
    /// assert_eq!(right, (3u32, 4u64, 5i8));
    /// ```
    fn split_right_at<Idx>(self) -> (Self::SplitLeftExclusive, Self::SplitRightInclusive)
    where
        Self: TypedTuple<Idx, T>,
    {
        <Self as TypedTuple<Idx, T>>::split_right(self)
    }

    #[inline]
    /// Splits the tuple at Idx (inclusive both), returning two tuples.
    ///
    /// The element at Idx is included in both tuples (cloned).
    ///
    /// # Example
    ///
    /// ```rust
    /// # use typed_tuple::*;
    /// let tuple = (1u8, 2u16, 3u32, 4u64, 5i8);
    /// let (left, right) = tuple.split_inclusive_at::<TupleIndex2>();
    /// assert_eq!(left, (1u8, 2u16, 3u32));
    /// assert_eq!(right, (3u32, 4u64, 5i8));
    /// ```
    fn split_inclusive_at<Idx>(self) -> (Self::SplitLeftInclusive, Self::SplitRightInclusive)
    where
        Self: TypedTuple<Idx, T>,
        T: Clone,
    {
        <Self as TypedTuple<Idx, T>>::split_inclusive(self)
    }
}

impl<T, TT> TypedTupleExt<T> for TT {}

// Generate all tuple implementations using the proc macro
#[cfg(not(feature = "len_128"))]
typed_tuple_macros::generate_typed_tuple_impls!(64);

#[cfg(feature = "len_128")]
typed_tuple_macros::generate_typed_tuple_impls!(128);
