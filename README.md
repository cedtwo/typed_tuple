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

The main purpose of this crate is to simplfy small arbitrary operations on heterogenous sequences.

### Basic Operations

```rust
use typed_tuple::TypedTuple;

let mut tuple: (i32, f64, String) = (0, 0.0, String::new());

// Get and modify elements by type
*tuple.get_mut() = 42i32;
*tuple.get_mut() = 3.14f64;
*tuple.get_mut() = "hello".to_string();

// Map elements with closures
tuple.map(|x: i32| x * 2);

// Replace values
let old_value = tuple.replace(2.718f64);
assert_eq!(old_value, 3.14);

assert_eq!(tuple, (84, 2.718, "hello".to_string()));
```

### Advanced Operations

```rust
use typed_tuple::TypedTuple;

let tuple = (1u8, 2u16, 3u32, 4u64);

// Pop element by type
let (popped, rest): (u32, _) = tuple.pop();
assert_eq!(popped, 3u32);
assert_eq!(rest, (1u8, 2u16, 4u64));

// Swap elements at different indices (same type)
let mut tuple = (1u32, "hello", 2u32, 'x', 3u32);
TypedTuple::<0, u32>::swap::<2>(&mut tuple);
assert_eq!(tuple, (2u32, "hello", 1u32, 'x', 3u32));

// Split tuple at a specific index
let tuple = (1u8, 2u16, 3u32, 4u64, 5i8);
let (left, right) = TypedTuple::<2, u32>::split_at(tuple);
assert_eq!(left, (1u8, 2u16, 3u32));
assert_eq!(right, (4u64, 5i8));

// Take element, replacing with default
let mut tuple = (String::from("hello"), 42i32, 3.14f64);
let value: String = tuple.take();
assert_eq!(value, "hello");
assert_eq!(tuple, (String::new(), 42, 3.14));
```

## Limitations

- Fields of the same type must still specify a constant index. This can be specified with, for example, `TypedTuple::<1, _>::get(&tuple)` where `1` is the element index, however this offers no advantage over simply calling `tuple.1`.
- `typed_tuple` can impact readability. Types should be explicit if not immediately obvious. Prefer `let a: usize = tuple.get()` over `let a = tuple.get()`.

## Features

- **`len_128`**: Extends support from 64 to 128 elements. Note that enabling this feature increases compilation time due to higher macro recursion depth.

License: MIT OR Apache-2.0
