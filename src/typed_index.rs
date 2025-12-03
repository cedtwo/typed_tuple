/// Trait for tuple element manipulation by type.
pub trait TypedIndex<const INDEX: usize, T> {
    /// Get a reference to the element of type `T`.
    /// # Example
    /// ```
    /// # use typed_tuple::TypedIndex;
    /// // Get by type.
    /// let tuple = ("a", 'b', 2usize);
    /// let a: &&str = tuple.get();
    /// let b: &char = tuple.get();
    /// let c: &usize = tuple.get();
    ///
    /// // Get by 'const' index.
    /// let a = TypedIndex::<0, _>::get(&tuple);
    /// let b = TypedIndex::<1, _>::get(&tuple);
    /// let c = TypedIndex::<2, _>::get(&tuple);
    /// ```
    fn get(&self) -> &T;

    /// Get a mutable reference to the element of type `T`.
    /// # Example
    /// ```
    /// # use typed_tuple::TypedIndex;
    /// // Mutate by type.
    /// let mut tuple = ("a", 'b', 2usize);
    /// *tuple.get_mut() = "c";
    /// *tuple.get_mut() = 'd';
    /// *tuple.get_mut() = 3usize;
    /// assert_eq!(tuple, ("c", 'd', 3));
    ///
    /// // Mutate by 'const' index.
    /// *TypedIndex::<0, _>::get_mut(&mut tuple) = "e";
    /// *TypedIndex::<1, _>::get_mut(&mut tuple) = 'f';
    /// *TypedIndex::<2, _>::get_mut(&mut tuple) = 4usize;
    /// assert_eq!(tuple, ("e", 'f', 4))
    /// ```
    fn get_mut(&mut self) -> &mut T;
}

macro_rules! impl_typed_tuple {
    (( $($generics:tt ),* ), [ $( $( $idx_tail:tt ),+ )? ], []) => {};

    (( $($generics:tt ),* ), [$idx_head:tt  $(, $idx_tail:tt )* ], [ $gen_head:tt $(, $gen_tail:tt )* ]) => {
        impl< $( $generics ),+ > TypedIndex<$idx_head, $gen_head> for ( $( $generics ),+ ) {
            fn get(&self) -> &$gen_head {
                &self.$idx_head
            }

            fn get_mut(&mut self) -> &mut $gen_head {
                &mut self.$idx_head
            }
        }
        impl_typed_tuple!(($( $generics ),* ), [ $( $idx_tail ),* ], [ $( $gen_tail ),* ]);
    };

    (( $($generics:tt),* )) => {
        impl_typed_tuple!(
            ( $( $generics ),* ),
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [ $( $generics ),* ]);
    }
}

impl_typed_tuple!((A, B));
impl_typed_tuple!((A, B, C));
impl_typed_tuple!((A, B, C, D));
impl_typed_tuple!((A, B, C, D, E));
impl_typed_tuple!((A, B, C, D, E, F));
impl_typed_tuple!((A, B, C, D, E, F, G));
impl_typed_tuple!((A, B, C, D, E, F, G, H));
impl_typed_tuple!((A, B, C, D, E, F, G, H, I));
impl_typed_tuple!((A, B, C, D, E, F, G, H, I, K));
impl_typed_tuple!((A, B, C, D, E, F, G, H, I, K, J));
impl_typed_tuple!((A, B, C, D, E, F, G, H, I, K, J, L));
