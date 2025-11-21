# Typed Tuple

[![Documentation](https://docs.rs/typed_tuple/badge.svg)](https://docs.rs/typed_tuple)
[![CI](https://github.com/cedtwo/typed_tuple/workflows/Rust%20CI/badge.svg)](https://github.com/cedtwo/typed_tuple/actions)
[![Security Audit](https://github.com/cedtwo/typed_tuple/workflows/Security%20Audit/badge.svg)](https://github.com/cedtwo/typed_tuple/actions)
[![Codecov](https://codecov.io/gh/cedtwo/typed_tuple/branch/main/graph/badge.svg)](https://codecov.io/gh/cedtwo/typed_tuple)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Downloads](https://img.shields.io/crates/d/typed_tuple.svg)](https://crates.io/crates/typed_tuple)

Type based operations on primitive tuple elements, with`no_std` compatibility and support for tuples from 1 to 64 elements (or up to 128 with the `len_128` feature).

## Functionality

`typed_tuple` allows for type safe operations on primitive tuple elements. In most cases, operations can be performed without specifying an index. Traits relying on the `TypedTuple` trait unfortunately do require index specification for disambiguation.

The main purpose of this crate is to simplfy small arbitrary operations on heterogenous sequences. In the example below, elements of a tuple are assigned and retrieved irrespective of indices:

```rust
use typed_tuple::TypedTuple;

let mut tuple: (usize, u32, Option<u32>, Option<i32>, Option<i64>) = Default::default();

// Mutate the `usize` element.
tuple.map(|el: usize| el + 10);;
// Assign the `Type` prefixed elements.
tuple.replace(78u32);
*tuple.get_mut() = Some(56u32);
*tuple.get_mut() = Some(78i32);

// Pass elements to their respective consumers.
if let Some(element) = tuple.get() { std::hint::black_box::<u32>(*element); }
if let Some(element) = tuple.get() { std::hint::black_box::<i32>(*element); }
if let Some(element) = tuple.get() { std::hint::black_box::<i64>(*element); }

assert_eq!(tuple, (10, 78, Some(56), Some(78), None));
```

## Limitations

- Fields of the same type must still specify a constant index. This can be specified with, for example, `TypedTuple::<1, _>::get(&tuple)` where `1` is the element index, however this offers no advantage over simply calling `tuple.1`.
- `typed_tuple` can impact readability. Types should be explicit if not immediately obvious. Prefer `let a: usize = tuple.get()` over `let a = tuple.get()`.

## Features

- **`len_128`**: Extends support from 64 to 128 elements. Note that enabling this feature increases compilation time due to higher macro recursion depth.

License: MIT OR Apache-2.0
