extern crate proc_macro;

use proc_macro::TokenStream;
use syn::*;

mod typed_bound;
mod typed_index;

/// Implement `TypedIndex` on tuples of fields less than or equal to the given
/// integer literal.
///
/// # Example
/// ```
/// impl_typed_index!(12); // Implement on tuples of 1 to 12 fields.
/// ```
#[proc_macro]
pub fn impl_typed_index(item: TokenStream) -> TokenStream {
    match parse_int(item).map_err(|e| e.into_compile_error()) {
        Ok(n) => (0..n + 1).fold(TokenStream::new(), |mut stream, i| {
            stream.extend(typed_index::impl_typed_index(i));
            stream
        }),
        Err(e) => e.into(),
    }
}

/// Implement `TypedBound` on tuples of fields less than or equal to the given
/// integer literal.
///
/// # Example
/// ```
/// impl_typed_bound!(12); // Implement on tuples of 1 to 12 fields.
/// ```
#[proc_macro]
pub fn impl_typed_bound(item: TokenStream) -> TokenStream {
    match parse_int(item).map_err(|e| e.into_compile_error()) {
        Ok(n) => (0..n + 1).fold(TokenStream::new(), |mut stream, i| {
            stream.extend(typed_bound::impl_typed_bound(i));
            stream
        }),
        Err(e) => e.into(),
    }
}

/// Parse an (unsigned) integer literal input.
fn parse_int(item: TokenStream) -> syn::Result<usize> {
    let lit = syn::parse::<ExprLit>(item)?;
    match lit.lit {
        Lit::Int(int) => int.base10_parse(),
        _ => Err(syn::Error::new_spanned(lit, "Expected integer literal")),
    }
}
