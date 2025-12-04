use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::Index;

/// Implement `TypedExtract` for a tuple of `n` elements.
pub(super) fn impl_typed_extract(n: usize) -> TokenStream {
    let indices = (0..n).map(|i| Index::from(i)).collect::<Vec<_>>();
    let generics = (0..n).map(|i| format_ident!("T{i}")).collect::<Vec<_>>();

    (0..n).fold(TokenStream::new(), |mut stream, i| {
        let index = &indices[i];

        (i + 1..=n).for_each(|j| {
            let idx_range= &indices[i..j];
            let generic_range = &generics[i..j];

            stream.extend(TokenStream::from(quote! {
                impl< #( #generics ),* > TypedExtract< #index, ( #( #generic_range, )* )> for ( #( #generics, )* ) {
                    fn extract(self) -> ( #( #generic_range, )* ) {
                        ( #( self.#idx_range, )* )
                    }
                }
            }));
        });
        stream
    })
}
