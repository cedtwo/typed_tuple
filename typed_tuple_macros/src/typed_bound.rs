use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::Index;

/// Implement `TypedBound` for a tuple of `n` elements.
pub(super) fn impl_typed_bound(n: usize) -> TokenStream {
    let generics = (0..n).map(|i| format_ident!("T{i}")).collect::<Vec<_>>();

    (0..n.saturating_sub(1)).fold(TokenStream::new(), |mut stream, i| {
        let index = Index::from(i);

        let idx_left = (0..i + 1).map(|i| Index::from(i)).collect::<Vec<_>>() ;
        let idx_right = (i + 1..n).map(|i| Index::from(i)).collect::<Vec<_>>() ;
        let generic_left = generics.iter().take(i + 1).collect::<Vec<_>>();
        let generic_right = generics.iter().skip(i + 1).collect::<Vec<_>>();

        stream.extend(TokenStream::from(quote! {
            impl< #( #generics ),* > TypedBound< #index, ( #( #generic_left, )* ), ( #( #generic_right, )* )> for ( #( #generics, )* ) {
                fn split(self) -> (( #( #generic_left, )* ), ( #( #generic_right, )* )) {
                    (
                        ( #( self.#idx_left, )* ),
                        ( #( self.#idx_right, )* )
                    )
                }
            }
        }));

        stream
    })
}
