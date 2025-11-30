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
use typed_tuple::prelude::*;

let mut tuple: (i32, f64, String) = (0, 0.0, String::new());

// Get and modify elements by type
*tuple.get_mut() = 42i32;
*tuple.get_mut() = 3.14f64;
*tuple.get_mut() = "hello".to_string();

// Apply transformations with closures
tuple.apply(|x: &mut i32| *x *= 2);

// Replace values
let old_value = tuple.replace(2.718f64);
assert_eq!(old_value, 3.14);

assert_eq!(tuple, (84, 2.718, "hello".to_string()));
```

### Advanced Operations

```rust
use typed_tuple::prelude::*;

let tuple = (1u8, 2u16, 3u32, 4u64);

// Pop element by type
let (popped, rest): (u32, _) = tuple.pop();
assert_eq!(popped, 3u32);
assert_eq!(rest, (1u8, 2u16, 4u64));

// Swap elements at different indices (same type)
let mut tuple = (1u32, "hello", 2u32, 'x', 3u32);
tuple.swap::<typenum::U0, typenum::U2>();
assert_eq!(tuple, (2u32, "hello", 1u32, 'x', 3u32));

// Split tuple exclusively (element separated)
let tuple = (1u8, 2u16, 3u32, 4u64, 5i8);
let (left, element, right) = tuple.split_exclusive::<typenum::U2>();
assert_eq!(left, (1u8, 2u16));
assert_eq!(element, 3u32);
assert_eq!(right, (4u64, 5i8));

// Split tuple with element on left
let tuple = (1u8, 2u16, 3u32, 4u64, 5i8);
let (left, right) = tuple.split_left::<typenum::U2>();
assert_eq!(left, (1u8, 2u16, 3u32));
assert_eq!(right, (4u64, 5i8));

// Split tuple with element on right
let tuple = (1u8, 2u16, 3u32, 4u64, 5i8);
let (left, right) = tuple.split_right::<typenum::U2>();
assert_eq!(left, (1u8, 2u16));
assert_eq!(right, (3u32, 4u64, 5i8));

// Take element, replacing with default
let mut tuple = (String::from("hello"), 42i32, 3.14f64);
let value: String = tuple.take();
assert_eq!(value, "hello");
assert_eq!(tuple, (String::new(), 42, 3.14));
```

### Using LastIndex to Access the Last Element

The `LastIndex` trait provides a type-safe way to access the last element of any tuple, regardless of its size. This is particularly useful when working with generic code.

```rust
use typed_tuple::prelude::*;

// Define tuple types
type Tuple2 = (u8, u16);
type Tuple5 = (u8, u16, u32, u64, i8);

// Get the index marker for the last element
type Last2 = <Tuple2 as LastIndex>::Last;
type Last5 = <Tuple5 as LastIndex>::Last;

// Access last elements using the markers
let tuple2: Tuple2 = (1, 2);
let last: &u16 = tuple2.get::<Last2>();
assert_eq!(*last, 2u16);

let tuple5: Tuple5 = (1, 2, 3, 4, 5);
let last: &i8 = tuple5.get::<Last5>();
assert_eq!(*last, 5i8);

// All operations work with LastIndex
type Tuple3 = (u8, u16, u32);
type Last3 = <Tuple3 as LastIndex>::Last;

let mut tuple: Tuple3 = (1, 2, 3);
tuple.apply::<Last3, _>(|x| *x *= 10);
assert_eq!(tuple, (1u8, 2u16, 30u32));

// Pop the last element
let tuple: (u8, u16, u32, u64) = (1, 2, 3, 4);
type LastIdx = <(u8, u16, u32, u64) as LastIndex>::Last;
let (last, rest) = tuple.pop::<LastIdx>();
assert_eq!(last, 4u64);
assert_eq!(rest, (1u8, 2u16, 3u32));
```

### Using TupleKey for Blanket Implementations

The `TupleKey` trait enables defining blanket implementations that work with different tuple structures. You can use custom marker types to create semantic, type-safe APIs that work across various tuple layouts.

```rust
use typed_tuple::prelude::*;

// Define a marker type for semantic access
struct AgeMarker;

// Create a trait that uses the marker
trait GetAge {
    fn age(&self) -> u8;
}

// Blanket implementation for any tuple that can be keyed by AgeMarker
impl<T> GetAge for T
where
    Self: TypedIndex<<AgeMarker as TupleKey<Self>>::Idx, u8>,
    AgeMarker: TupleKey<Self>,
{
    fn age(&self) -> u8 {
        *self.get()
    }
}

// Map the marker to different indices for different tuple structures
impl TupleKey<(u8, f64, &str)> for AgeMarker {
    type Idx = typenum::U0;
}

impl TupleKey<(&str, f64, u8)> for AgeMarker {
    type Idx = typenum::U2;
}

impl TupleKey<(&str, u8, f64)> for AgeMarker {
    type Idx = typenum::U1;
}

// Now you can call .age() on different tuple structures
assert_eq!((67u8, 3.5, "Alice").age(), 67u8);
assert_eq!(("Bob", 2.1, 42u8).age(), 42u8);
assert_eq!(("Charlie", 56u8, 1.8).age(), 56u8);
```

## Limitations

- Fields of the same type must still specify an index type. This can be specified with, for example, `TypedTuple::<typenum::U1, _>::get(&tuple)` where `typenum::U1` is the element index type, however this offers no advantage over simply calling `tuple.1`.
- `typed_tuple` can impact readability. Types should be explicit if not immediately obvious. Prefer `let a: usize = tuple.get()` over `let a = tuple.get()`.

## Features

- **`len_128`**: Extends support from 64 to 128 elements. Note that enabling this feature increases compilation time due to higher macro recursion depth.

License: MIT OR Apache-2.0
