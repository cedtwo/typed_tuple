//! Tests for TupleIndex arithmetic operations

use typed_tuple::prelude::*;

#[test]
fn test_index_add() {
    // Basic addition
    type Result1 = <TupleIndex0 as TupleIndexAdd<TupleIndex0>>::Output;
    assert_eq!(<Result1 as TupleIndex>::Idx, 0);

    type Result2 = <TupleIndex2 as TupleIndexAdd<TupleIndex3>>::Output;
    assert_eq!(<Result2 as TupleIndex>::Idx, 5);

    type Result3 = <TupleIndex10 as TupleIndexAdd<TupleIndex5>>::Output;
    assert_eq!(<Result3 as TupleIndex>::Idx, 15);

    // Addition with zero
    type Result4 = <TupleIndex0 as TupleIndexAdd<TupleIndex20>>::Output;
    assert_eq!(<Result4 as TupleIndex>::Idx, 20);

    type Result5 = <TupleIndex7 as TupleIndexAdd<TupleIndex0>>::Output;
    assert_eq!(<Result5 as TupleIndex>::Idx, 7);
}

#[test]
fn test_index_sub() {
    // Basic subtraction
    type Result1 = <TupleIndex5 as TupleIndexSub<TupleIndex2>>::Output;
    assert_eq!(<Result1 as TupleIndex>::Idx, 3);

    type Result2 = <TupleIndex10 as TupleIndexSub<TupleIndex3>>::Output;
    assert_eq!(<Result2 as TupleIndex>::Idx, 7);

    // Subtraction to zero
    type Result3 = <TupleIndex7 as TupleIndexSub<TupleIndex7>>::Output;
    assert_eq!(<Result3 as TupleIndex>::Idx, 0);

    // Subtraction with zero
    type Result4 = <TupleIndex15 as TupleIndexSub<TupleIndex0>>::Output;
    assert_eq!(<Result4 as TupleIndex>::Idx, 15);
}

#[test]
fn test_index_arithmetic_composition() {
    // (5 + 3) - 2 = 6
    type Step1 = <TupleIndex5 as TupleIndexAdd<TupleIndex3>>::Output;
    type Step2 = <Step1 as TupleIndexSub<TupleIndex2>>::Output;
    assert_eq!(<Step2 as TupleIndex>::Idx, 6);

    // (10 - 4) + 8 = 14
    type Step3 = <TupleIndex10 as TupleIndexSub<TupleIndex4>>::Output;
    type Step4 = <Step3 as TupleIndexAdd<TupleIndex8>>::Output;
    assert_eq!(<Step4 as TupleIndex>::Idx, 14);
}

#[test]
fn test_index_arithmetic_with_tuple_access() {
    let tuple = (0u8, 1u16, 2u32, 3u64, 4i8, 5i16, 6i32, 7i64);

    // Access at index 2 + 3 = 5
    type Idx = <TupleIndex2 as TupleIndexAdd<TupleIndex3>>::Output;
    let value: &i16 = tuple.get::<Idx>();
    assert_eq!(*value, 5i16);

    // Access at index 7 - 2 = 5
    type Idx2 = <TupleIndex7 as TupleIndexSub<TupleIndex2>>::Output;
    let value2: &i16 = tuple.get::<Idx2>();
    assert_eq!(*value2, 5i16);
}

#[test]
fn test_large_index_arithmetic() {
    // Test with larger indices
    type Result1 = <TupleIndex30 as TupleIndexAdd<TupleIndex20>>::Output;
    assert_eq!(<Result1 as TupleIndex>::Idx, 50);

    type Result2 = <TupleIndex63 as TupleIndexSub<TupleIndex10>>::Output;
    assert_eq!(<Result2 as TupleIndex>::Idx, 53);

    type Result3 = <TupleIndex50 as TupleIndexSub<TupleIndex50>>::Output;
    assert_eq!(<Result3 as TupleIndex>::Idx, 0);
}
