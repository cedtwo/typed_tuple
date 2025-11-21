use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitInt};

/// Generates TypedTuple implementations for tuples up to the specified size.
///
/// This proc macro generates all the necessary trait implementations including
/// the `pop` method for each tuple size and index combination.
#[proc_macro]
pub fn generate_typed_tuple_impls(input: TokenStream) -> TokenStream {
    let max_size = parse_macro_input!(input as LitInt);
    let max_size: usize = max_size.base10_parse().expect("Expected a number");

    let mut impls = Vec::new();

    // Generate implementation for each tuple size
    for size in 1..=max_size {
        let type_params: Vec<_> = (0..size).map(|i| quote::format_ident!("T{}", i)).collect();

        // Generate implementation for each index in the tuple
        for (index, target_type) in type_params.iter().enumerate() {
            let index_lit = syn::Index::from(index);

            // Build type and index lists for pop, split_at
            let pop_output_types =
                type_params.iter().enumerate().filter(|(i, _)| *i != index).map(|(_, t)| t);
            let remaining_indices = (0..size).filter(|i| *i != index).map(syn::Index::from);

            let split_left_types = type_params.iter().take(index + 1);
            let split_right_types = type_params.iter().skip(index + 1);
            let split_left_indices = (0..=index).map(syn::Index::from);
            let split_right_indices = ((index + 1)..size).map(syn::Index::from);

            impls.push(quote! {
                impl<#(#type_params),*> TypedTuple<#index, #target_type> for (#(#type_params,)*) {
                    type PopOutput = (#(#pop_output_types,)*);
                    type SplitLeft = (#(#split_left_types,)*);
                    type SplitRight = (#(#split_right_types,)*);

                    #[inline]
                    fn get(&self) -> &#target_type {
                        &self.#index_lit
                    }

                    #[inline]
                    fn get_mut(&mut self) -> &mut #target_type {
                        &mut self.#index_lit
                    }
                    #[inline]
                    fn map<FN: FnOnce(#target_type) -> #target_type>(&mut self, f: FN)
                    where
                        #target_type: Default
                    {
                        self.#index_lit = f(core::mem::take(&mut self.#index_lit));
                    }
                    #[inline]
                    fn pop(self) -> (#target_type, Self::PopOutput) {
                        (self.#index_lit, (#(self.#remaining_indices,)*))
                    }
                    #[inline]
                    fn swap<const OTHER_INDEX: usize>(&mut self)
                    where
                        Self: TypedTuple<OTHER_INDEX, #target_type>
                    {
                        if #index != OTHER_INDEX {
                            unsafe {
                                let ptr = self as *mut Self;
                                let field1 = &mut (*ptr).#index_lit;
                                let field2 = <Self as TypedTuple<OTHER_INDEX, #target_type>>::get_mut(&mut *ptr);
                                core::mem::swap(field1, field2);
                            }
                        }
                    }
                    #[inline]
                    fn split_at(self) -> (Self::SplitLeft, Self::SplitRight) {
                        ((#(self.#split_left_indices,)*), (#(self.#split_right_indices,)*))
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
