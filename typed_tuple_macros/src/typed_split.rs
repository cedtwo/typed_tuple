use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::Index;

/// Implement `TypedSplit` for a tuple of `n` elements.
pub(super) fn impl_typed_split(n: usize) -> TokenStream {
    let indices = (0..n + 1).map(|i| Index::from(i)).collect::<Vec<_>>();
    let generics = (0..n).map(|i| format_ident!("T{i}")).collect::<Vec<_>>();

    (0..n + 1).fold(TokenStream::new(), |mut stream, i| {
        let index = &indices[i];

        let idx_left = (0..i).map(|i| Index::from(i)).collect::<Vec<_>>();
        let idx_right = (i..n).map(|i| Index::from(i)).collect::<Vec<_>>();

        let generic_left = generics.iter().take(i).collect::<Vec<_>>();
        let generic_right = generics.iter().skip(i).collect::<Vec<_>>();

        stream.extend(TokenStream::from(quote! {
            impl< #( #generics ),* > TypedSplit< #index, ( #( #generic_left, )* ), ( #( #generic_right, )* )> for ( #( #generics, )* ) {
                fn split(self) -> (( #( #generic_left, )* ), ( #( #generic_right, )* )) {
                    (
                        ( #( self.#idx_left, )* ),
                        ( #( self.#idx_right, )* )
                    )
                }
            }

            impl<'a,  #( #generics ),* > TypedSplit< #index, ( #( &'a #generic_left, )* ), ( #( &'a #generic_right, )* )> for &'a ( #( #generics, )* ) {
                fn split(self) -> (( #( &'a #generic_left, )* ), ( #( &'a #generic_right, )* )) {
                    (
                        ( #( &self.#idx_left, )* ),
                        ( #( &self.#idx_right, )* )
                    )
                }
            }

            impl<'a,  #( #generics ),* > TypedSplit< #index, ( #( &'a mut #generic_left, )* ), ( #( &'a mut #generic_right, )* )> for &'a mut ( #( #generics, )* ) {
                fn split(self) -> (( #( &'a mut #generic_left, )* ), ( #( &'a mut #generic_right, )* )) {
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
