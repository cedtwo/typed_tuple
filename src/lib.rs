//! # Typed Tuple
//! Type based operations on primitive tuple elements.
//!
//! ## Functionality
//!
//! `typed_tuple` allows for type safe operations on primitive tuple elements
//! without specifying an index. The main purpose of this crate is to simplfy
//! small arbitrary operations on heterogenous sequences. In the example below,
//! elements of a tuple are assigned and retrieved irrespective of indices:
//!
//! ```
//! # use typed_tuple::TypedIndex;
//! # #[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
//! # struct Type0;
//! # #[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
//! # struct Type1;
//! # #[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
//! # struct Type2;
//! let mut tuple: (usize, Option<Type0>, Option<Type1>, Option<Type2>) = Default::default();
//!
//! // Assign values by type.
//! *tuple.get_mut() = 10;
//! *tuple.get_mut() = Some(Type0);
//! *tuple.get_mut() = Some(Type2);
//!
//! // Pass elements to their respective consumers.
//! if let Some(element) = tuple.get() { std::hint::black_box::<Type0>(*element); }
//! if let Some(element) = tuple.get() { std::hint::black_box::<Type1>(*element); }
//! if let Some(element) = tuple.get() { std::hint::black_box::<Type2>(*element); }
//!
//! assert_eq!(tuple, (10, Some(Type0), None, Some(Type2)));
//! ```
//!
//! ## Limitations
//!
//! - Fields of the same type must still specify a constant index. This can be specified
//! with, for example, `TypedIndex::<1, _>::get(&tuple)` where `1` is the element index,
//! however this offers no advantage over simply calling `tuple.1`.
//! - `typed_tuple` can impact readability. Types should be explicit if not immediately
//! obvious. Prefer `let a: usize = tuple.get()` over `let a = tuple.get()`.
//! - `TypedIndex` is implemented on tuples of up to 12 elements in length. This was chosen
//! as it is the limit of many tuple trait implementations (`PartialEq`, `Eq`, etc.),
//! however can be extended to support a higher number of elements if needed.

mod typed_extract;
mod typed_index;
mod typed_split;

pub use typed_extract::TypedExtract;
pub use typed_index::TypedIndex;
pub use typed_split::TypedSplit;
