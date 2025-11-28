//! Submodule for TypedTuple extension traits.

use crate::prelude::*;

/// Extension trait to add additional methods to TypedTuple.
pub trait TypedTuple<T>: Sized {
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
    /// # use typed_tuple::prelude::IndexedTuple;
    /// let mut tuple = (10u32, 20u64);
    /// let old = tuple.replace(30u32);
    /// assert_eq!(old, 10u32);
    /// assert_eq!(tuple, (30u32, 20u64));
    /// ```
    #[inline]
    fn replace<INDEX: TupleIndex>(&mut self, value: T) -> T
    where
        Self: IndexedTuple<INDEX, T>,
        INDEX: TupleIndex,
    {
        core::mem::replace(self.get_mut(), value)
    }

    /// Takes a closure, mutating the element of type `T`.
    /// # Example
    /// ```
    /// # use typed_tuple::prelude::*;
    /// // Apply by type.
    /// let mut tuple = ("a".to_string(), 1u8, 2usize);
    /// tuple.apply(|el: &mut String| *el = el.to_uppercase());
    /// tuple.apply(|el: &mut u8| *el += 1);
    /// tuple.apply(|el: &mut usize| *el += 2);
    /// assert_eq!(tuple, ("A".to_string(), 2, 4));
    ///
    /// // Apply by 'const' index.
    /// IndexedTuple::<TupleIndex0, _>::apply(&mut tuple, |el| *el = el.to_lowercase());
    /// IndexedTuple::<TupleIndex1, _>::apply(&mut tuple, |el| *el -= 1);
    /// IndexedTuple::<TupleIndex2, _>::apply(&mut tuple, |el| *el -= 2);
    /// assert_eq!(tuple, ("a".to_string(), 1, 2))
    /// ```
    fn apply<INDEX: TupleIndex, FN: FnOnce(&mut T)>(&mut self, f: FN)
    where
        Self: IndexedTuple<INDEX, T>,
        INDEX: TupleIndex,
    {
        f(self.get_mut());
    }

    /// Returns the outcome of the provided closure when applied to the element
    /// of type `T`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use typed_tuple::prelude::*;
    ///
    /// let tuple = (10u32, 20u64);
    /// let result = tuple.map(|x: &u32| x * 2);
    /// assert_eq!(result, 20u32);
    ///
    /// let result = tuple.map(|x: &u64| x + 5);
    /// assert_eq!(result, 25u64);
    /// ```
    fn map<INDEX: TupleIndex, FN, R>(&self, f: FN) -> R
    where
        Self: IndexedTuple<INDEX, T>,
        INDEX: TupleIndex,
        FN: FnOnce(&T) -> R,
    {
        f(self.get())
    }

    /// Returns the outcome of the provided closure when applied to a mutable
    /// reference to the element of type `T`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use typed_tuple::prelude::*;
    ///
    /// let mut tuple = (10u32, 20u64);
    /// let result = tuple.map_mut(|x: &mut u32| {
    ///     *x += 5;
    ///     *x * 2
    /// });
    /// assert_eq!(result, 30u32);
    /// assert_eq!(tuple, (15u32, 20u64));
    ///
    /// let result = tuple.map_mut(|x: &mut u64| {
    ///     *x *= 2;
    ///     *x
    /// });
    /// assert_eq!(result, 40u64);
    /// assert_eq!(tuple, (15u32, 40u64));
    /// ```
    fn map_mut<INDEX: TupleIndex, FN, R>(&mut self, f: FN) -> R
    where
        Self: IndexedTuple<INDEX, T>,
        INDEX: TupleIndex,
        FN: FnOnce(&mut T) -> R,
    {
        f(self.get_mut())
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
    /// # use typed_tuple::prelude::*;
    /// // Pop by type.
    /// let tuple = ("a", 'b', 2usize);
    /// let (s, rest): (&str, _) = tuple.pop();
    /// assert_eq!(s, "a");
    /// assert_eq!(rest, ('b', 2usize));
    ///
    /// // Pop by 'const' index.
    /// let tuple = ("a", 'b', 2usize);
    /// let (c, rest) = IndexedTuple::<TupleIndex1, _>::pop(tuple);
    /// assert_eq!(c, 'b');
    /// assert_eq!(rest, ("a", 2usize));
    /// ```
    fn pop<INDEX: TupleIndex>(self) -> (T, Self::PopOutput)
    where
        Self: IndexedTuple<INDEX, T>,
        INDEX: TupleIndex,
    {
        let (left, element, right) = self.split_exclusive();
        (element, left.chain_right(right))
    }

    #[inline]
    /// Swaps the element at INDEX with the element at OTHER_INDEX.
    ///
    /// Both indices must contain elements of type `T`. If INDEX == OTHER_INDEX,
    /// this is a no-op.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use typed_tuple::prelude::*;
    /// let mut tuple = (1u32, "hello", 2u32, 'x', 3u32);
    /// IndexedTuple::<TupleIndex0, u32>::swap::<TupleIndex2>(&mut tuple);
    /// assert_eq!(tuple, (2u32, "hello", 1u32, 'x', 3u32));
    /// ```
    fn swap<INDEX, OTHER>(&mut self)
    where
        Self: IndexedTuple<INDEX, T> + IndexedTuple<OTHER, T>,
        INDEX: TupleIndex,
        OTHER: TupleIndex,
    {
        if INDEX::INDEX != OTHER::INDEX {
            unsafe {
                let ptr = self as *mut Self;
                let field1 = <Self as IndexedTuple<INDEX, T>>::get_mut(&mut *ptr);
                let field2 = <Self as IndexedTuple<OTHER, T>>::get_mut(&mut *ptr);
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
    /// # use typed_tuple::prelude::IndexedTuple;
    /// let mut tuple = (String::from("hello"), 42, 3.14);
    /// let s: String = tuple.take();
    /// assert_eq!(s, "hello");
    /// assert_eq!(tuple, (String::new(), 42, 3.14));
    /// ```
    #[inline]
    fn take<INDEX>(&mut self) -> T
    where
        Self: IndexedTuple<INDEX, T>,
        INDEX: TupleIndex,
        T: Default,
    {
        core::mem::take(self.get_mut())
    }

    #[inline]
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
    /// # use typed_tuple::prelude::*;
    /// let tuple = (1u8, 2u16, 3u32, 4u64, 5i8);
    /// let (left, right) = IndexedTuple::<TupleIndex2, u32>::split_left(tuple);
    /// assert_eq!(left, (1u8, 2u16, 3u32));
    /// assert_eq!(right, (4u64, 5i8));
    /// ```
    fn split_left<INDEX>(self) -> (Self::SplitLeftInclusive, Self::SplitRightExclusive)
    where
        Self: IndexedTuple<INDEX, T>,
        INDEX: TupleIndex,
    {
        let (left_exclusive, element, right_exclusive) = self.split_exclusive();
        (left_exclusive.chain_right((element,)), right_exclusive)
    }

    #[inline]
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
    /// # use typed_tuple::prelude::*;
    /// let tuple = (1u8, 2u16, 3u32, 4u64, 5i8);
    /// let (left, right) = IndexedTuple::<TupleIndex2, u32>::split_right(tuple);
    /// assert_eq!(left, (1u8, 2u16));
    /// assert_eq!(right, (3u32, 4u64, 5i8));
    /// ```
    fn split_right<INDEX>(self) -> (Self::SplitLeftExclusive, Self::SplitRightInclusive)
    where
        Self: IndexedTuple<INDEX, T>,
        INDEX: TupleIndex,
    {
        let (left_exclusive, element, right_exclusive) = self.split_exclusive();
        (left_exclusive, right_exclusive.chain_left((element,)))
    }

    #[inline]
    /// Splits the tuple at INDEX (inclusive both sides), returning two tuples.
    ///
    /// The element at INDEX is cloned and included in both tuples.
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
    /// # use typed_tuple::prelude::*;
    /// let tuple = (1u8, 2u16, 3u32, 4u64, 5i8);
    /// let (left, right) = IndexedTuple::<TupleIndex2, u32>::split_inclusive(tuple);
    /// assert_eq!(left, (1u8, 2u16, 3u32));
    /// assert_eq!(right, (3u32, 4u64, 5i8));
    /// ```
    fn split_inclusive<INDEX>(self) -> (Self::SplitLeftInclusive, Self::SplitRightInclusive)
    where
        Self: IndexedTuple<INDEX, T>,
        INDEX: TupleIndex,
        T: Clone,
    {
        let (left_exclusive, element, right_exclusive) = self.split_exclusive();
        (
            left_exclusive.chain_right((element.clone(),)),
            right_exclusive.chain_left((element,)),
        )
    }
}

impl<T, TT> TypedTuple<T> for TT {}
