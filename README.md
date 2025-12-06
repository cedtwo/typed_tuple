# Typed Tuple
Type-safe access, isolation and mutation of primitive tuple segments and elements.
Supports both type and/or numeric indexing for **all** operations.

## Functionality

`typed_tuple` enables typed or numeric index access to tuple elements. All operations
support access by either numeric index/indices, or type inferred access, Some use-cases
for these operations may include:
- Getting an element by specifying a unique type (eg. `u32`).
- Getting a subset of elements by specifying a unique type range (eg. `(u32, _, _, _)`).
- Getting or splitting the first *n* elements (eg. `((_, _, _), _)`).
- Getting or splitting the last *n* elements (eg. `(_, (_, _, _))`).
- Any combination of the above.

### Example

```rust
# use typed_tuple::{TypedIndex, TypedSplit, TypedExtract};
let mut tuple = (0u8, 1u16, 2u32, 3u64, 'a', 'b', 'c');

// Get the (unique) `u64` element.
let element_u64: u64 = tuple.get();
assert_eq!(element_u64, 3);

// Mutate the (unique) `u16` element.
let mut element_u16: &mut u16 = (&mut tuple).get();
*element_u16 = 100;
assert_eq!(element_u16, &mut 100);

// Split the tuple returning a right segment of exactly 3 elements.
let (left, right): (_, (_, _, _)) = tuple.split();
assert_eq!(left, (0, 100, 2, 3));
assert_eq!(right, ('a', 'b', 'c'));

// Extract (with mutable borrow) the 3 elements starting at the `u32` field.
let mut extracted: (&mut u32, &mut _, &mut _) = (&mut tuple).extract();
assert_eq!(extracted, (&mut 2, &mut 3, &mut 'a'));

// Get the (unique) `&mut char` type from the extracted segment.
let mut ext_char: &mut char = extracted.get();
*ext_char = 'x';

assert_eq!(tuple, (0u8, 100u16, 2u32, 3u64, 'x', 'b', 'c'));
```

## Operation Arguments

Elements are accessed by an index/indices inferred by the return type. Explict
numeric indices can be specified in cases where type inferred indices are not
possible/preferable. See trait-level documentation for examples of explicit/inferred
index operations.

### Example

```rust
# use typed_tuple::{TypedSplit};
let mut tuple = (0u8, 1u16, 2u32, 3u64, 4u128);

let (left_inferred, right_inferred): (_, (_,)) = tuple.split(); // Split the last element off the tuple.
let (left_explicit, right_explicit) = TypedSplit::<4, _, _>::split(tuple); // Split the tuple at index 4 (left-exclusive).

assert_eq!(left_inferred, left_explicit);
assert_eq!(right_inferred, right_explicit);
```

## Ownership

Operations can either return owned, borrowed or mutable borrowed elements. Element
state follows the borrow state of the tuple. Given a tuple `Self` containing the
element `T`, operations exhibit the following behaviour:

Tuple State | Element State | Behaviour | Example
---|---|---|---
`Self` | `T` | Consume the tuple returning owned element(s) | `tuple.get()`
`&Self` | `&T` | Borrow the tuple returning borrowed element(s) | `(&tuple).get()`
`&mut Self` | `&mut T` | Borrow the tuple returning mutable borrowed element(s) | `(&mut tuple).get()`

## Limitations

- `typed_tuple` can impact readability. Types should be explicit if not immediately
obvious. Prefer `let a: usize = tuple.get()` over `let a = tuple.get()`.
- `TypedTuple` is implemented on tuples of up to 12 elements in length. This was chosen
as it is the limit of many tuple trait implementations (`PartialEq`, `Eq`, etc.),
however can be extended to support a higher number of elements if needed.
