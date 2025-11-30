//! Submodule for trait bound mapping.
use crate::prelude::*;

/// Extension trait to add additional methods to TypedTuple.
pub trait TypedTuple<T>: Sized {
    /// Get a reference to the element of type `T`.
    /// # Example
    /// ```
    /// # use typed_tuple::prelude::*;
    /// // Get by type.
    /// let tuple = ("a", 'b', 2usize);
    /// let a: &&str = tuple.get();
    /// let b: &char = tuple.get();
    /// let c: &usize = tuple.get();
    /// assert_eq!((a, b, c), (&"a", &'b', &2));
    ///
    /// // Get by index.
    /// let a = tuple.get::<TupleIndex0>();
    /// let b = tuple.get::<TupleIndex1>();
    /// let c = tuple.get::<TupleIndex2>();
    /// assert_eq!((a, b, c), (&"a", &'b', &2));
    /// ```
    #[inline]
    fn get<Idx>(&self) -> &T
    where
        Self: TypedIndex<Idx, T>,
        Idx: TupleIndex,
    {
        TypedIndex::<Idx, T>::get_at(self)
    }

    /// Get a mutable reference to the element of type `T`.
    /// # Example
    /// ```
    /// # use typed_tuple::prelude::*;
    /// // Mutate by type.
    /// let mut tuple = ("a", 'b', 2usize);
    /// *tuple.get_mut() = "c";
    /// *tuple.get_mut() = 'd';
    /// *tuple.get_mut() = 3usize;
    /// assert_eq!(tuple, ("c", 'd', 3));
    ///
    /// // Mutate by index.
    /// *tuple.get_mut::<TupleIndex0>() = "e";
    /// *tuple.get_mut::<TupleIndex1>() = 'f';
    /// *tuple.get_mut::<TupleIndex2>() = 4usize;
    /// assert_eq!(tuple, ("e", 'f', 4))
    /// ```
    #[inline]
    fn get_mut<Idx>(&mut self) -> &mut T
    where
        Self: TypedIndex<Idx, T>,
        Idx: TupleIndex,
    {
        TypedIndex::<Idx, T>::get_mut_at(self)
    }

    /// Splits the tuple exclusively at Idx, returning the element and the
    /// surrounding tuples.
    ///
    /// # Returns
    ///
    /// A tuple containing (left_exclusive, element, right_exclusive) where:
    /// - `left_exclusive` contains elements [0..Idx)
    /// - `element` is the element at Idx
    /// - `right_exclusive` contains elements (Idx..)
    ///
    /// # Example
    ///
    /// ```rust
    /// # use typed_tuple::prelude::*;
    /// let tuple = (1u8, 2u16, 3u32, 4u64, 5i8);
    /// let (left, element, right) = tuple.split_exclusive::<TupleIndex2>();
    /// assert_eq!(left, (1u8, 2u16));
    /// assert_eq!(element, 3u32);
    /// assert_eq!(right, (4u64, 5i8));
    /// ```
    #[inline]
    fn split_exclusive<Idx>(self) -> (Self::SplitLeftExclusive, T, Self::SplitRightExclusive)
    where
        Self: TypedBounds<Idx, T>,
        Idx: TupleIndex,
    {
        TypedBounds::<Idx, T>::split_exclusive_at(self)
    }

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
    /// # use typed_tuple::prelude::*;
    /// let mut tuple = (10u32, 20u64);
    /// let old = tuple.replace(30u32);
    /// assert_eq!(old, 10u32);
    /// assert_eq!(tuple, (30u32, 20u64));
    /// ```
    #[inline]
    fn replace<Idx>(&mut self, value: T) -> T
    where
        Self: TypedIndex<Idx, T>,
        Idx: TupleIndex,
    {
        core::mem::replace(self.get_mut::<Idx>(), value)
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
    /// // Apply by index.
    /// tuple.apply::<TupleIndex0, _>(|el| *el = el.to_lowercase());
    /// tuple.apply::<TupleIndex1, _>(|el| *el -= 1);
    /// tuple.apply::<TupleIndex2, _>(|el| *el -= 2);
    /// assert_eq!(tuple, ("a".to_string(), 1, 2))
    /// ```
    fn apply<Idx, FN: FnOnce(&mut T)>(&mut self, f: FN)
    where
        Self: TypedIndex<Idx, T>,
        Idx: TupleIndex,
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
    /// // Map by type.
    /// let tuple = (10u32, 20u64);
    /// let result = tuple.map(|x: &u32| x * 2);
    /// assert_eq!(result, 20u32);
    ///
    /// // Map by index.
    /// let result = tuple.map::<TupleIndex1, _, _>(|x| x + 5);
    /// assert_eq!(result, 25u64);
    /// ```
    fn map<Idx, FN, R>(&self, f: FN) -> R
    where
        Self: TypedIndex<Idx, T>,
        Idx: TupleIndex,
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
    /// // Map by type.
    /// let mut tuple = (10u32, 20u64);
    /// let result = tuple.map_mut(|x: &mut u32| {
    ///     *x += 5;
    ///     *x * 2
    /// });
    /// assert_eq!(result, 30u32);
    /// assert_eq!(tuple, (15u32, 20u64));
    ///
    /// // Map by index.
    /// let result = tuple.map_mut::<TupleIndex1, _, _>(|x| {
    ///     *x *= 2;
    ///     *x
    /// });
    /// assert_eq!(result, 40u64);
    /// assert_eq!(tuple, (15u32, 40u64));
    /// ```
    fn map_mut<Idx, FN, R>(&mut self, f: FN) -> R
    where
        Self: TypedIndex<Idx, T>,
        Idx: TupleIndex,
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
    /// // Pop by index.
    /// let tuple = ("a", 'b', 2usize);
    /// let (c, rest) = tuple.pop::<TupleIndex1>();
    /// assert_eq!(c, 'b');
    /// assert_eq!(rest, ("a", 2usize));
    /// ```
    fn pop<Idx>(self) -> (T, Self::PopOutput)
    where
        Self: TypedBounds<Idx, T>,
        Idx: TupleIndex,
    {
        let (left, element, right) = self.split_exclusive();
        (element, left.chain_right(right))
    }

    /// Swaps the element at `Idx` with the element at `Other`.
    ///
    /// Both indices must contain elements of type `T`. If `Idx` == `Other`,
    /// this is a no-op.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use typed_tuple::prelude::*;
    /// let mut tuple = (1u32, "hello", 2u32, 'x', 3u32);
    /// tuple.swap::<TupleIndex0, TupleIndex2>();
    /// assert_eq!(tuple, (2u32, "hello", 1u32, 'x', 3u32));
    /// ```
    #[inline]
    fn swap<Idx, Other>(&mut self)
    where
        Self: TypedIndex<Idx, T> + TypedIndex<Other, T>,
        Idx: TupleIndex,
        Other: TupleIndex,
    {
        if Idx::Idx != Other::Idx {
            unsafe {
                let ptr = self as *mut Self;
                let field1 = (&mut *ptr).get_mut::<Idx>();
                let field2 = (&mut *ptr).get_mut::<Other>();
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
    /// # use typed_tuple::prelude::*;
    /// let mut tuple = (String::from("hello"), 42, 3.14);
    /// let s: String = tuple.take();
    /// assert_eq!(s, "hello");
    /// assert_eq!(tuple, (String::new(), 42, 3.14));
    /// ```
    #[inline]
    fn take<Idx>(&mut self) -> T
    where
        Self: TypedIndex<Idx, T>,
        Idx: TupleIndex,
        T: Default,
    {
        core::mem::take(self.get_mut())
    }

    #[inline]
    /// Splits the tuple at Idx (inclusive left), returning two tuples.
    ///
    /// The element at Idx is included in the left tuple.
    ///
    /// # Returns
    ///
    /// A tuple containing (left_inclusive, right_exclusive) where:
    /// - `left_inclusive` contains elements [0..=Idx]
    /// - `right_exclusive` contains elements (Idx..)
    ///
    /// # Example
    ///
    /// ```rust
    /// # use typed_tuple::prelude::*;
    /// let tuple = (1u8, 2u16, 3u32, 4u64, 5i8);
    /// let (left, right) = tuple.split_left::<TupleIndex2>();
    /// assert_eq!(left, (1u8, 2u16, 3u32));
    /// assert_eq!(right, (4u64, 5i8));
    /// ```
    fn split_left<Idx>(self) -> (Self::SplitLeftInclusive, Self::SplitRightExclusive)
    where
        Self: TypedBounds<Idx, T>,
        Idx: TupleIndex,
    {
        let (left_exclusive, element, right_exclusive) = self.split_exclusive();
        (left_exclusive.chain_right((element,)), right_exclusive)
    }

    #[inline]
    /// Splits the tuple at Idx (inclusive right), returning two tuples.
    ///
    /// The element at Idx is included in the right tuple.
    ///
    /// # Returns
    ///
    /// A tuple containing (left_exclusive, right_inclusive) where:
    /// - `left_exclusive` contains elements [0..Idx)
    /// - `right_inclusive` contains elements [Idx..]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use typed_tuple::prelude::*;
    /// let tuple = (1u8, 2u16, 3u32, 4u64, 5i8);
    /// let (left, right) = tuple.split_right::<TupleIndex2>();
    /// assert_eq!(left, (1u8, 2u16));
    /// assert_eq!(right, (3u32, 4u64, 5i8));
    /// ```
    fn split_right<Idx>(self) -> (Self::SplitLeftExclusive, Self::SplitRightInclusive)
    where
        Self: TypedBounds<Idx, T>,
        Idx: TupleIndex,
    {
        let (left_exclusive, element, right_exclusive) = self.split_exclusive();
        (left_exclusive, right_exclusive.chain_left((element,)))
    }

    #[inline]
    /// Splits the tuple at Idx (inclusive both sides), returning two tuples.
    ///
    /// The element at Idx is cloned and included in both tuples.
    ///
    /// # Returns
    ///
    /// A tuple containing (left_inclusive, right_inclusive) where:
    /// - `left_inclusive` contains elements [0..=Idx]
    /// - `right_inclusive` contains elements [Idx..]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use typed_tuple::prelude::*;
    /// let tuple = (1u8, 2u16, 3u32, 4u64, 5i8);
    /// let (left, right) = tuple.split_inclusive::<TupleIndex2>();
    /// assert_eq!(left, (1u8, 2u16, 3u32));
    /// assert_eq!(right, (3u32, 4u64, 5i8));
    /// ```
    fn split_inclusive<Idx>(self) -> (Self::SplitLeftInclusive, Self::SplitRightInclusive)
    where
        Self: TypedBounds<Idx, T>,
        Idx: TupleIndex,
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
