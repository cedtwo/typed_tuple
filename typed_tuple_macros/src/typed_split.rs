use proc_macro::TokenStream;
use quote::quote;
use syn::{Ident, Index};

/// Implement `TypedSplit` for a tuple of `n` elements.
pub(super) fn impl_typed_split(
    n: usize,
    indices: &Vec<Index>,
    generics: &Vec<Ident>,
) -> TokenStream {
    let indices = &indices[0..n + 1];
    let generics = &generics[0..n];

    (0..n + 1).fold(TokenStream::new(), |mut stream, i| {
        let index = &indices[i];

        let (idx_left, idx_right) = indices[..n].split_at(i);
        let (gen_left, gen_right) = generics[..n].split_at(i);

        let all_generics = quote! { #( #generics, )* };

        let return_ty_args_own = quote! { ( #( #gen_left, )* ), ( #( #gen_right, )* ) };
        let return_ty_inner_ref = quote! { ( #( &'a #gen_left, )* ), ( #( &'a #gen_right, )* ) };
        let return_ty_inner_mut = quote! { ( #( &'a mut #gen_left, )* ), ( #( &'a mut #gen_right, )* ) };

        stream.extend(TokenStream::from(quote! {
            impl< #all_generics > TypedSplit< #index, #return_ty_args_own > for ( #all_generics ) {
                fn split(self) -> ( #return_ty_args_own ) {
                    (
                        ( #( self.#idx_left, )* ),
                        ( #( self.#idx_right, )* )
                    )
                }
            }

            impl<'a, #all_generics > TypedSplit< #index, #return_ty_inner_ref > for &'a ( #all_generics ) {
                fn split(self) -> ( #return_ty_inner_ref ) {
                    (
                        ( #( &self.#idx_left, )* ),
                        ( #( &self.#idx_right, )* )
                    )
                }
            }

            impl<'a, #all_generics > TypedSplit< #index, #return_ty_inner_mut > for &'a mut ( #all_generics ) {
                fn split(self) -> ( #return_ty_inner_mut ) {
                    (
                        ( #( &mut self.#idx_left, )* ),
                        ( #( &mut self.#idx_right, )* )
                    )
                }
            }
        }));

        stream
    })
}
