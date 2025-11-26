//! Proc macros to generate TypedTuple implementations for tuples up to a
//! specified size.
//!
//! This crate is split into multiple macros to avoid hitting token limits:
//! - `generate_index_markers!()` - Generates TupleIndexN marker types
//! - `generate_last_index_impls!()` - Generates LastIndex trait implementations
//! - `generate_typed_tuple_impls!()` - Generates TypedTuple trait
//!   implementations
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

/// Generates index marker types (TupleIndex0, TupleIndex1, etc.)
#[proc_macro]
pub fn generate_index_markers(_input: TokenStream) -> TokenStream {
    let mut impls = Vec::new();

    for index in 0..MAX_SIZE {
        let marker_ident = quote::format_ident!("TupleIndex{}", index);
        let documentation = format!("Marker type for tuple index {}", index);
        impls.push(quote! {
            #[doc = #documentation]
            pub struct #marker_ident;

            impl TupleIndex for #marker_ident {
                const INDEX: usize = #index;
            }
        });
    }

    quote! {
        #(#impls)*
    }
    .into()
}

/// Generates LastIndex trait implementations for all tuple sizes
#[proc_macro]
pub fn generate_last_index_impls(_input: TokenStream) -> TokenStream {
    let mut impls = Vec::new();

    for size in 1..=MAX_SIZE {
        let type_params: Vec<_> = (0..size).map(|i| quote::format_ident!("T{}", i)).collect();
        let last_index = size - 1;
        let last_marker = quote::format_ident!("TupleIndex{}", last_index);

        impls.push(quote! {
            impl<#(#type_params),*> LastIndex for (#(#type_params,)*) {
                type Last = #last_marker;
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
            let index_marker = quote::format_ident!("TupleIndex{}", index);

            impls.push(quote! {
                impl<#(#type_params),*> NthIndex<#index_marker> for (#(#type_params,)*) {
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

/// Generates TupleIndexAdd trait implementations for valid index combinations
#[proc_macro]
pub fn generate_index_add_impls(_input: TokenStream) -> TokenStream {
    let mut impls = Vec::new();

    for left in 0..MAX_SIZE {
        for right in 0..MAX_SIZE {
            let sum = left + right;
            if sum >= MAX_SIZE {
                continue;
            }

            let left_marker = quote::format_ident!("TupleIndex{}", left);
            let right_marker = quote::format_ident!("TupleIndex{}", right);
            let output_marker = quote::format_ident!("TupleIndex{}", sum);

            impls.push(quote! {
                impl TupleIndexAdd<#right_marker> for #left_marker {
                    type Output = #output_marker;
                }
            });
        }
    }

    quote! {
        #(#impls)*
    }
    .into()
}

/// Generates TupleIndexSub trait implementations for valid index combinations
#[proc_macro]
pub fn generate_index_sub_impls(_input: TokenStream) -> TokenStream {
    let mut impls = Vec::new();

    for left in 0..MAX_SIZE {
        for right in 0..=left {
            let diff = left - right;

            let left_marker = quote::format_ident!("TupleIndex{}", left);
            let right_marker = quote::format_ident!("TupleIndex{}", right);
            let output_marker = quote::format_ident!("TupleIndex{}", diff);

            impls.push(quote! {
                impl TupleIndexSub<#right_marker> for #left_marker {
                    type Output = #output_marker;
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

            let left_params: Vec<_> =
                (0..left_size).map(|i| quote::format_ident!("L{}", i)).collect();
            let right_params: Vec<_> =
                (0..right_size).map(|i| quote::format_ident!("R{}", i)).collect();

            let left_indices: Vec<_> = (0..left_size).map(syn::Index::from).collect();
            let right_indices: Vec<_> = (0..right_size).map(syn::Index::from).collect();

            impls.push(quote! {
                impl<#(#left_params,)* #(#right_params,)*> ChainRight<(#(#right_params,)*)> for (#(#left_params,)*) {
                    type Output = (#(#left_params,)* #(#right_params,)*);

                    #[inline]
                    fn chain_right(self, right: (#(#right_params,)*)) -> Self::Output {
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

/// Generates TypedTuple trait implementations for all tuple sizes and indices
#[proc_macro]
pub fn generate_typed_tuple_impls(_input: TokenStream) -> TokenStream {
    let mut impls = Vec::new();

    for size in 1..=MAX_SIZE {
        let type_params: Vec<_> = (0..size).map(|i| quote::format_ident!("T{}", i)).collect();

        // Generate implementation for each index in the tuple
        for index in 0..size {
            let target_type = &type_params[index];
            let index_marker = quote::format_ident!("TupleIndex{}", index);
            let index_lit = syn::Index::from(index);

            // Build type lists for split operations
            let split_left_exclusive_types = type_params.iter().take(index);
            let split_right_exclusive_types = type_params.iter().skip(index + 1);
            let split_left_exclusive_indices = (0..index).map(syn::Index::from);
            let split_right_exclusive_indices = ((index + 1)..size).map(syn::Index::from);

            impls.push(quote! {
                impl<#(#type_params),*> TypedTuple<#index_marker, #target_type> for (#(#type_params,)*) {
                    type PopOutput = <Self::SplitLeftExclusive as ChainRight<Self::SplitRightExclusive>>::Output;
                    type SplitLeftExclusive = (#(#split_left_exclusive_types,)*);
                    type SplitLeftInclusive = <Self::SplitLeftExclusive as ChainRight<(#target_type,)>>::Output;
                    type SplitRightExclusive = (#(#split_right_exclusive_types,)*);
                    type SplitRightInclusive = <(#target_type,) as ChainRight<Self::SplitRightExclusive>>::Output;

                    #[inline]
                    fn get(&self) -> &#target_type {
                        &self.#index_lit
                    }
                    #[inline]
                    fn get_mut(&mut self) -> &mut #target_type {
                        &mut self.#index_lit
                    }
                    #[inline]
                    fn split_exclusive(self) -> (Self::SplitLeftExclusive, #target_type, Self::SplitRightExclusive) {
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
