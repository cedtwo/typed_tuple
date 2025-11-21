#![doc = include_str!("../README.md")]
#![no_std]
#![cfg_attr(feature = "len_128", recursion_limit = "256")]

/// Trait for tuple element manipulation by type.
pub trait TypedTuple<const INDEX: usize, T> {
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

    /// Sets the provided element of type `T`.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to set the element to.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use typed_tuple::TypedTuple;
    /// let mut tuple = (10u32, 20u64);
    /// tuple.set(30u32);
    /// tuple.set(40u64);
    /// assert_eq!(tuple, (30u32, 40u64));
    /// ```
    fn set(&mut self, value: T) {
        *self.get_mut() = value;
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
}

macro_rules! impl_typed_tuple {
    // Base case: no more generics to process
    (( $($generics:tt ),* ), [ $( $( $idx_tail:tt ),+ )? ], []) => {};

    // Recursive case: implement for current type and recurse
    (( $($generics:tt ),* ), [$idx_head:tt  $(, $idx_tail:tt )* ], [ $gen_head:tt $(, $gen_tail:tt )* ]) => {
        impl< $( $generics ),+ > TypedTuple<$idx_head, $gen_head> for ( $( $generics ),+ ) {
            fn get(&self) -> &$gen_head {
                &self.$idx_head
            }

            fn get_mut(&mut self) -> &mut $gen_head {
                &mut self.$idx_head
            }

            fn map<FN: FnOnce($gen_head) -> $gen_head>(&mut self, f: FN) where $gen_head: Default {
                self.$idx_head = f(core::mem::take(&mut self.$idx_head));
            }
        }
        impl_typed_tuple!(($( $generics ),* ), [ $( $idx_tail ),* ], [ $( $gen_tail ),* ]);
    };

    // Entry point for 2+ element tuples
    (( $g1:tt, $g2:tt $(, $grest:tt)* )) => {
        #[cfg(not(feature = "len_128"))]
        impl_typed_tuple!(
            ( $g1, $g2 $(, $grest)* ),
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63],
            [ $g1, $g2 $(, $grest)* ]);

        #[cfg(feature = "len_128")]
        impl_typed_tuple!(
            ( $g1, $g2 $(, $grest)* ),
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127],
            [ $g1, $g2 $(, $grest)* ]);
    };

    // Entry point for 1-element tuple - special handling to preserve tuple syntax
    (($t:tt)) => {
        impl<$t> TypedTuple<0, $t> for ($t,) {
            fn get(&self) -> &$t {
                &self.0
            }

            fn get_mut(&mut self) -> &mut $t {
                &mut self.0
            }

            fn map<FN: FnOnce($t) -> $t>(&mut self, f: FN) where $t: Default {
                self.0 = f(core::mem::take(&mut self.0));
            }
        }
    };
}

// Generate implementations recursively from 1 to 64 (or 128 with feature) elements
macro_rules! impl_all_tuples {
    // Base case: implement for 1-tuple and stop
    ($g1:tt) => {
        impl_typed_tuple!(($g1));
    };

    // Recursive case: implement for current tuple size, then recurse with one less generic
    ($g1:tt, $($rest:tt),+) => {
        impl_typed_tuple!(($g1, $($rest),+));
        impl_all_tuples!($($rest),+);
    };
}

// Generate all tuple implementations from 1 to 64 elements (default)
#[cfg(not(feature = "len_128"))]
impl_all_tuples!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21,
    T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40,
    T41, T42, T43, T44, T45, T46, T47, T48, T49, T50, T51, T52, T53, T54, T55, T56, T57, T58, T59,
    T60, T61, T62, T63, T64
);

// Generate all tuple implementations from 1 to 128 elements (with len_128 feature)
#[cfg(feature = "len_128")]
impl_all_tuples!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21,
    T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40,
    T41, T42, T43, T44, T45, T46, T47, T48, T49, T50, T51, T52, T53, T54, T55, T56, T57, T58, T59,
    T60, T61, T62, T63, T64, T65, T66, T67, T68, T69, T70, T71, T72, T73, T74, T75, T76, T77, T78,
    T79, T80, T81, T82, T83, T84, T85, T86, T87, T88, T89, T90, T91, T92, T93, T94, T95, T96, T97,
    T98, T99, T100, T101, T102, T103, T104, T105, T106, T107, T108, T109, T110, T111, T112, T113,
    T114, T115, T116, T117, T118, T119, T120, T121, T122, T123, T124, T125, T126, T127, T128
);
