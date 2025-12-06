use proc_macro::TokenStream;
use quote::quote;
use syn::{Ident, Index};

/// Implement `TypedIndex` for a tuple of `n` elements.
pub(super) fn impl_typed_index(
    n: usize,
    indices: &Vec<Index>,
    generics: &Vec<Ident>,
) -> TokenStream {
    let indices = &indices[..n];
    let generics = &generics[..n];

    (0..n).fold(TokenStream::new(), |mut stream, i| {
        let idx = &indices[i];
        let gen_ty = &generics[i];

        let all_generics = quote! { #( #generics, )* };

        stream.extend(TokenStream::from(quote! {
            impl< #all_generics > TypedIndex< #idx, #gen_ty> for ( #all_generics ) {
                fn get(self) -> #gen_ty {
                    self.#idx
                }
            }

            impl<'a, #all_generics > TypedIndex< #idx, &'a #gen_ty> for &'a ( #all_generics ) {
                fn get(self) -> &'a #gen_ty {
                    &self.#idx
                }
            }

            impl<'a, #all_generics > TypedIndex< #idx, &'a mut #gen_ty> for &'a mut ( #all_generics ) {
                fn get(self) -> &'a mut #gen_ty {
                    &mut self.#idx
                }
            }
        }));

        stream
    })
}
