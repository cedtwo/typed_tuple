#![doc = include_str!("../README.md")]
#[cfg(feature = "extract")]
mod typed_extract;
#[cfg(feature = "index")]
mod typed_index;
#[cfg(feature = "split")]
mod typed_split;

#[cfg(feature = "extract")]
pub use typed_extract::TypedExtract;
#[cfg(feature = "index")]
pub use typed_index::TypedIndex;
#[cfg(feature = "split")]
pub use typed_split::TypedSplit;
