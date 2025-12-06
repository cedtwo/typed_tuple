use proc_macro::TokenStream;
use quote::{quote};
use syn::{Ident, Index};

/// Implement `TypedExtract` for a tuple of `n` elements.
pub(super) fn impl_typed_extract(n: usize, indices: &Vec<Index>, generics: &Vec<Ident>) -> TokenStream {
    let indices = &indices[..n + 1];
    let generics = &generics[..n];

    (0..n).fold(TokenStream::new(), |mut stream, i| {

        (i..=n).for_each(|j| {
            let index_start = &indices[i];
            let index_end = &indices[j];

            let extract_range_idx = &indices[i..j];
            let extract_range_gen = &generics[i..j];

            let all_generics = quote! { #( #generics, )* };

            let return_ty_own = quote! { ( #( #extract_range_gen, )* ) };
            let return_ty_ref = quote! { ( #( &'a #extract_range_gen, )* ) };
            let return_ty_mut = quote! { ( #( &'a mut #extract_range_gen, )* ) };

            stream.extend(TokenStream::from(quote! {
                impl< #all_generics > TypedExtract< #index_start, #index_end, #return_ty_own > for ( #all_generics ) {
                    fn extract(self) -> #return_ty_own {
                        ( #( self.#extract_range_idx, )* )
                    }
                }

                impl<'a, #all_generics > TypedExtract< #index_start, #index_end, #return_ty_ref > for &'a ( #all_generics ) {
                    fn extract(self) -> #return_ty_ref {
                        ( #( &self.#extract_range_idx, )* )
                    }
                } 

                impl<'a, #all_generics > TypedExtract< #index_start, #index_end, #return_ty_mut > for &'a mut ( #all_generics ) {
                    fn extract(self) -> #return_ty_mut {
                        ( #( &mut self.#extract_range_idx, )* )
                    }
                } 
            }));
        });
        stream
    })
}
