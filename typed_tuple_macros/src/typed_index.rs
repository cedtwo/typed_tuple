use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::Index;

/// Implement `TypedIndex` for a tuple of `n` elements.
pub(super) fn impl_typed_index(n: usize) -> TokenStream {
    let generics = (0..n).map(|i| format_ident!("T{i}")).collect::<Vec<_>>();

    (0..n).fold(TokenStream::new(), |mut stream, i| {
        let index = Index::from(i);
        let generic = &generics[i as usize];

        stream.extend(TokenStream::from(quote! {
            impl< #( #generics ),* > TypedIndex< #index, #generic> for ( #( #generics, )* ) {
                fn get(&self) -> &#generic {
                    &self.#index
                }

                fn get_mut(&mut self) -> &mut #generic {
                    &mut self.#index
                }
            }
        }));

        stream
    })
}
