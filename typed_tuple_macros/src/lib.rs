//! Proc macros to generate TypedTuple implementations for tuples up to a
//! specified size.
//!
//! This crate provides macros to generate trait implementations:
//! - `generate_last_index_impls!()` - Generates LastIndex trait implementations
//! - `generate_nth_index_impls!()` - Generates NthIndex trait implementations
//! - `generate_typed_tuple_impls!()` - Generates TypedIndex and TypedBounds trait implementations
//! - `generate_chain_right_impls!()` - Generates ChainRight trait implementations
//!
//! The maximum tuple size is controlled by the `len_128` feature flag:
//! - Without feature: supports tuples up to 64 elements
//! - With `len_128` feature: supports tuples up to 128 elements
use proc_macro::TokenStream;
use quote::quote;

/// Determine the maximum tuple size based on the len_128 feature
#[cfg(not(feature = "len_128"))]
const MAX_SIZE: usize = 64;

#[cfg(feature = "len_128")]
const MAX_SIZE: usize = 128;

/// Generates LastIndex trait implementations for all tuple sizes
#[proc_macro]
pub fn generate_last_index_impls(_input: TokenStream) -> TokenStream {
    let mut impls = Vec::new();

    for size in 1..=MAX_SIZE {
        let type_params: Vec<_> = (0..size).map(|i| quote::format_ident!("T{}", i)).collect();
        let last_index = size - 1;
        let last_marker = quote::format_ident!("U{}", last_index);

        impls.push(quote! {
            impl<#(#type_params),*> LastIndex for (#(#type_params,)*) {
                type Last = typenum::#last_marker;
            }
        });
    }

    quote! {
        #(#impls)*
    }
    .into()
}

/// Generates NthIndex trait implementations for all tuples and valid indices
#[proc_macro]
pub fn generate_nth_index_impls(_input: TokenStream) -> TokenStream {
    let mut impls = Vec::new();

    for size in 1..=MAX_SIZE {
        let type_params: Vec<_> = (0..size).map(|i| quote::format_ident!("T{}", i)).collect();

        // Generate implementation for each valid index in the tuple
        for index in 0..size {
            let nth_type = &type_params[index];
            let index_marker = quote::format_ident!("U{}", index);

            impls.push(quote! {
                impl<#(#type_params),*> NthIndex<typenum::#index_marker> for (#(#type_params,)*) {
                    type NthType = #nth_type;
                }
            });
        }
    }

    quote! {
        #(#impls)*
    }
    .into()
}

/// Generates ChainRight trait implementations for all tuple size combinations
#[proc_macro]
pub fn generate_chain_right_impls(_input: TokenStream) -> TokenStream {
    let mut impls = Vec::new();

    // Generate ChainRight implementations for all combinations of tuple sizes
    for left_size in 0..=MAX_SIZE {
        for right_size in 0..=MAX_SIZE {
            let total_size = left_size + right_size;
            if total_size > MAX_SIZE {
                continue;
            }

            let left_params: Vec<_> = (0..left_size)
                .map(|i| quote::format_ident!("L{}", i))
                .collect();
            let right_params: Vec<_> = (0..right_size)
                .map(|i| quote::format_ident!("R{}", i))
                .collect();

            let left_indices: Vec<_> = (0..left_size).map(syn::Index::from).collect();
            let right_indices: Vec<_> = (0..right_size).map(syn::Index::from).collect();

            impls.push(quote! {
                impl<#(#left_params,)* #(#right_params,)*> ChainRight<(#(#right_params,)*)> for (#(#left_params,)*) {
                    type Output = (#(#left_params,)* #(#right_params,)*);

                    #[inline]
                    fn chain_right(self, right: (#(#right_params,)*)) -> Self::Output {
                        #[allow(unused_unit)]
                        (#(self.#left_indices,)* #(right.#right_indices,)*)
                    }
                }
            });
        }
    }

    quote! {
        #(#impls)*
    }
    .into()
}

/// Generates TypedIndex trait implementations for all tuple sizes and indices
#[proc_macro]
pub fn generate_typed_tuple_impls(_input: TokenStream) -> TokenStream {
    let mut impls = Vec::new();

    for size in 1..=MAX_SIZE {
        let type_params: Vec<_> = (0..size).map(|i| quote::format_ident!("T{}", i)).collect();

        // Generate implementation for each index in the tuple
        for index in 0..size {
            let target_type = &type_params[index];
            let index_marker = quote::format_ident!("U{}", index);
            let index_lit = syn::Index::from(index);

            // Build type lists for split operations
            let split_left_exclusive_types = type_params.iter().take(index);
            let split_right_exclusive_types = type_params.iter().skip(index + 1);
            let split_left_exclusive_indices = (0..index).map(syn::Index::from);
            let split_right_exclusive_indices = ((index + 1)..size).map(syn::Index::from);

            impls.push(quote! {
                impl<#(#type_params),*> TypedIndex<typenum::#index_marker, #target_type> for (#(#type_params,)*) {

                    #[inline]
                    fn get_at(&self) -> &#target_type {
                        &self.#index_lit
                    }
                    #[inline]
                    fn get_mut_at(&mut self) -> &mut #target_type {
                        &mut self.#index_lit
                    }
                }

                impl<#(#type_params),*> TypedBounds<typenum::#index_marker, #target_type> for (#(#type_params,)*) {
                    type PopOutput = <Self::SplitLeftExclusive as ChainRight<Self::SplitRightExclusive>>::Output;
                    type SplitLeftExclusive = (#(#split_left_exclusive_types,)*);
                    type SplitLeftInclusive = <Self::SplitLeftExclusive as ChainRight<(#target_type,)>>::Output;
                    type SplitRightExclusive = (#(#split_right_exclusive_types,)*);
                    type SplitRightInclusive = <(#target_type,) as ChainRight<Self::SplitRightExclusive>>::Output;

                    #[inline]
                    fn split_exclusive_at(self) -> (Self::SplitLeftExclusive, #target_type, Self::SplitRightExclusive) {
                        ((#(self.#split_left_exclusive_indices,)*), self.#index_lit, (#(self.#split_right_exclusive_indices,)*))
                    }
                }
            });
        }
    }

    quote! {
        #(#impls)*
    }
    .into()
}

/// Generates the TypedLast trait definition with all required TypedNth bounds
#[proc_macro]
pub fn define_typed_last_trait(_input: TokenStream) -> TokenStream {
    let mut bounds = Vec::new();

    // Add the basic bound
    bounds.push(quote! { LastIndex<NthType = T> });

    // Add TypedNth bounds for Last and Last - i for all i
    bounds.push(quote! { TypedNth<<Self as LastIndex>::Last> });

    // Add the final TypedIndex bound
    bounds.push(quote! {
        TypedIndex<
            <Self as LastIndex>::Last,
            T,
            SplitRightInclusive = (T,),
            SplitLeftInclusive = Self,
            SplitRightExclusive = (),
        >
    });

    quote! {
        /// Trait for accessing the last element of a tuple by type.
        ///
        /// This trait is implemented for tuples where the last element is of type `T`.
        /// It combines the functionality of `LastIndex` and `TypedIndex` to provide
        /// type-safe access to the last element.
        ///
        /// # Examples
        ///
        /// ```rust
        /// # use typed_tuple::prelude::*;
        /// fn get_last<TT: TypedLast<u32>>(tuple: &TT) -> &u32 {
        ///     tuple.get()
        /// }
        ///
        /// let tuple = (1u8, 2u16, 3u32);
        /// assert_eq!(*get_last(&tuple), 3u32);
        ///
        /// let tuple2 = ("hello", 'x', 42u32);
        /// assert_eq!(*get_last(&tuple2), 42u32);
        /// ```
        pub trait TypedLast<T>: #(#bounds)+* {
        }
    }
    .into()
}

/// Generates the TypedLast trait implementation
#[proc_macro]
pub fn impl_typed_last_trait(_input: TokenStream) -> TokenStream {
    let mut bounds = Vec::new();

    // Add the basic bound
    bounds.push(quote! { LastIndex<NthType = T> });

    // Add TypedNth bounds for Last and Last - i for all i
    bounds.push(quote! { TypedNth<<TT as LastIndex>::Last> });

    // Add the final TypedIndex bound
    bounds.push(quote! {
        TypedIndex<
            <TT as LastIndex>::Last,
            T,
            SplitRightInclusive = (T,),
            SplitLeftInclusive = TT,
            SplitRightExclusive = (),
        >
    });

    quote! {
        impl<T, TT> TypedLast<T> for TT where TT: #(#bounds)+* {
        }
    }
    .into()
}

/// Generates the NthIndexedAs trait implementation
#[proc_macro]
pub fn impl_nth_indexed_as_trait(_input: TokenStream) -> TokenStream {
    let mut bounds = Vec::new();

    // Add the basic bound
    bounds.push(quote! { NthIndexedUntil<Idx> });

    // Add type equality bound for Idx
    bounds.push(quote! {
        NthIndex<Idx, NthType = <Other as NthIndex<Idx>>::NthType>
    });

    quote! {
        impl<Idx: typenum::Unsigned, Other: NthIndexedUntil<Idx>, TT> NthIndexedAs<Idx, Other> for TT
        where
            TT: #(#bounds)+*
        {
        }
    }
    .into()
}
