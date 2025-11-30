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
                const Idx: usize = #index;
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

/// Generates TupleIndexSaturatingSub trait implementations for all index
/// combinations
#[proc_macro]
pub fn generate_index_saturating_sub_impls(_input: TokenStream) -> TokenStream {
    let mut impls = Vec::new();

    for left in 0..MAX_SIZE {
        for right in 0..MAX_SIZE {
            let diff = if left >= right { left - right } else { 0 };

            let left_marker = quote::format_ident!("TupleIndex{}", left);
            let right_marker = quote::format_ident!("TupleIndex{}", right);
            let output_marker = quote::format_ident!("TupleIndex{}", diff);

            impls.push(quote! {
                impl TupleIndexSaturatingSub<#right_marker> for #left_marker {
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
            let index_marker = quote::format_ident!("TupleIndex{}", index);
            let index_lit = syn::Index::from(index);

            // Build type lists for split operations
            let split_left_exclusive_types = type_params.iter().take(index);
            let split_right_exclusive_types = type_params.iter().skip(index + 1);
            let split_left_exclusive_indices = (0..index).map(syn::Index::from);
            let split_right_exclusive_indices = ((index + 1)..size).map(syn::Index::from);

            impls.push(quote! {
                impl<#(#type_params),*> TypedIndex<#index_marker, #target_type> for (#(#type_params,)*) {

                    #[inline]
                    fn get_at(&self) -> &#target_type {
                        &self.#index_lit
                    }
                    #[inline]
                    fn get_mut_at(&mut self) -> &mut #target_type {
                        &mut self.#index_lit
                    }
                }

                impl<#(#type_params),*> TypedBounds<#index_marker, #target_type> for (#(#type_params,)*) {
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

/// Generates the TupleIndex trait definition with all required
/// TupleIndexSaturatingSub bounds
#[proc_macro]
pub fn define_tuple_index_trait(_input: TokenStream) -> TokenStream {
    let mut bounds = Vec::new();

    // Add the basic bounds
    bounds.push(quote! { Sized });
    bounds.push(quote! { TupleIndexSub<Self, Output = TupleIndex0> });
    bounds.push(quote! { TupleIndexSub<TupleIndex0, Output = Self> });
    bounds.push(quote! { TupleIndexAdd<TupleIndex0, Output = Self> });

    // Add TupleIndexSaturatingSub bounds for all indices
    for i in 0..MAX_SIZE {
        let index_marker = quote::format_ident!("TupleIndex{}", i);
        bounds.push(quote! { TupleIndexSaturatingSub<#index_marker> });
    }

    quote! {
        /// Trait for tuple index types.
        pub trait TupleIndex: #(#bounds)+* {
            /// The associated index value.
            const Idx: usize;
        }
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

    for i in 1..MAX_SIZE {
        let index_marker = quote::format_ident!("TupleIndex{}", i);
        bounds.push(quote! {
            TypedNth<<<Self as LastIndex>::Last as TupleIndexSaturatingSub<#index_marker>>::Output>
        });
    }

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

    for i in 1..MAX_SIZE {
        let index_marker = quote::format_ident!("TupleIndex{}", i);
        bounds.push(quote! {
            TypedNth<<<TT as LastIndex>::Last as TupleIndexSaturatingSub<#index_marker>>::Output>
        });
    }

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

/// Generates the TypedUntil trait definition with all required TypedNth bounds
#[proc_macro]
pub fn define_typed_until_trait(_input: TokenStream) -> TokenStream {
    let mut bounds = Vec::new();

    // Add the basic bound
    bounds.push(quote! { NthIndex<Idx> });

    // Add TypedNth bounds for Idx and Idx - i for all i
    bounds.push(quote! { TypedNth<Idx> });

    for i in 1..MAX_SIZE {
        let index_marker = quote::format_ident!("TupleIndex{}", i);
        bounds.push(quote! {
            TypedNth<<Idx as TupleIndexSaturatingSub<#index_marker>>::Output>
        });
    }

    // Add the TypedIndex bound
    bounds.push(quote! {
        TypedIndex<Idx, <Self as NthIndex<Idx>>::NthType>
    });

    quote! {
        /// Trait for accessing elements of a tuple up to a specific index.
        ///
        /// This trait is implemented for tuples that have elements accessible
        /// up to and including index `Idx`. It provides bounds for all indices
        /// from 0 up to `Idx` using saturating subtraction.
        ///
        /// This trait is automatically implemented for all tuples where the index
        /// is valid, and it ensures that all elements from index 0 up to `Idx`
        /// can be safely accessed.
        pub trait TypedUntil<Idx: TupleIndex>: #(#bounds)+* {
        }
    }
    .into()
}

/// Generates the TypedUntil trait implementation
#[proc_macro]
pub fn impl_typed_until_trait(_input: TokenStream) -> TokenStream {
    let mut bounds = Vec::new();

    // Add the basic bound
    bounds.push(quote! { NthIndex<Idx> });

    // Add TypedNth bounds for Idx and Idx - i for all i
    bounds.push(quote! { TypedNth<Idx> });

    for i in 1..MAX_SIZE {
        let index_marker = quote::format_ident!("TupleIndex{}", i);
        bounds.push(quote! {
            TypedNth<<Idx as TupleIndexSaturatingSub<#index_marker>>::Output>
        });
    }

    // Add the TypedIndex bound
    bounds.push(quote! {
        TypedIndex<Idx, <TT as NthIndex<Idx>>::NthType>
    });

    quote! {
        impl<Idx: TupleIndex, TT> TypedUntil<Idx> for TT where TT: #(#bounds)+* {
        }
    }
    .into()
}

/// Generates the NthIndexedUntil trait definition
#[proc_macro]
pub fn define_nth_indexed_until_trait(_input: TokenStream) -> TokenStream {
    let mut bounds = Vec::new();

    // Add NthIndex bounds for Idx and all saturating subtractions
    bounds.push(quote! { NthIndex<Idx> });

    for i in 1..MAX_SIZE {
        let index_marker = quote::format_ident!("TupleIndex{}", i);
        bounds.push(quote! {
            NthIndex<<Idx as TupleIndexSaturatingSub<#index_marker>>::Output>
        });
    }

    quote! {
        /// Trait for tuples that implement `NthIndex` up to a specific index.
        ///
        /// This trait ensures that a tuple has `NthIndex` implementations for
        /// index `Idx` and all indices reachable by saturating subtraction from `Idx`.
        pub trait NthIndexedUntil<Idx: TupleIndex>: #(#bounds)+* {
        }
    }
    .into()
}

/// Generates the NthIndexedUntil trait implementation
#[proc_macro]
pub fn impl_nth_indexed_until_trait(_input: TokenStream) -> TokenStream {
    let mut bounds = Vec::new();

    // Add NthIndex bounds for Idx and all saturating subtractions
    bounds.push(quote! { NthIndex<Idx> });

    for i in 1..MAX_SIZE {
        let index_marker = quote::format_ident!("TupleIndex{}", i);
        bounds.push(quote! {
            NthIndex<<Idx as TupleIndexSaturatingSub<#index_marker>>::Output>
        });
    }

    quote! {
        impl<Idx: TupleIndex, TT> NthIndexedUntil<Idx> for TT
        where
            TT: #(#bounds)+*
        {
        }
    }
    .into()
}

/// Generates the NthIndexedAs trait definition
#[proc_macro]
pub fn define_nth_indexed_as_trait(_input: TokenStream) -> TokenStream {
    let mut bounds = Vec::new();

    // Add the basic bound
    bounds.push(quote! { NthIndexedUntil<Idx> });

    // Add type equality bounds for Idx and all saturating subtractions
    bounds.push(quote! {
        NthIndex<Idx, NthType = <TT as NthIndex<Idx>>::NthType>
    });

    for i in 1..MAX_SIZE {
        let index_marker = quote::format_ident!("TupleIndex{}", i);
        bounds.push(quote! {
            NthIndex<
                <Idx as TupleIndexSaturatingSub<#index_marker>>::Output,
                NthType = <TT as NthIndex<<Idx as TupleIndexSaturatingSub<#index_marker>>::Output>>::NthType
            >
        });
    }

    quote! {
        /// Trait for tuples with matching element types up to a specific index.
        ///
        /// This trait is implemented for tuples where all element types from index 0
        /// up to and including index `Idx` match the corresponding types in tuple `TT`.
        pub trait NthIndexedAs<Idx: TupleIndex, TT: NthIndexedUntil<Idx>>: #(#bounds)+* {
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

    // Add type equality bounds for Idx and all saturating subtractions
    bounds.push(quote! {
        NthIndex<Idx, NthType = <Other as NthIndex<Idx>>::NthType>
    });

    for i in 1..MAX_SIZE {
        let index_marker = quote::format_ident!("TupleIndex{}", i);
        bounds.push(quote! {
            NthIndex<
                <Idx as TupleIndexSaturatingSub<#index_marker>>::Output,
                NthType = <Other as NthIndex<<Idx as TupleIndexSaturatingSub<#index_marker>>::Output>>::NthType
            >
        });
    }

    quote! {
        impl<Idx: TupleIndex, Other: NthIndexedUntil<Idx>, TT> NthIndexedAs<Idx, Other> for TT
        where
            TT: #(#bounds)+*
        {
        }
    }
    .into()
}
