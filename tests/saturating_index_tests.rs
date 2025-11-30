//! Tests for TupleIndex saturating arithmetic operations

use typed_tuple::prelude::*;

#[test]
fn test_saturating_sub_basic() {
    // Basic saturating subtraction (no saturation)
    type Result1 = <TupleIndex5 as TupleIndexSaturatingSub<TupleIndex2>>::Output;
    assert_eq!(<Result1 as TupleIndex>::Idx, 3);

    type Result2 = <TupleIndex10 as TupleIndexSaturatingSub<TupleIndex3>>::Output;
    assert_eq!(<Result2 as TupleIndex>::Idx, 7);

    // Subtraction to zero
    type Result3 = <TupleIndex7 as TupleIndexSaturatingSub<TupleIndex7>>::Output;
    assert_eq!(<Result3 as TupleIndex>::Idx, 0);

    // Subtraction with zero
    type Result4 = <TupleIndex15 as TupleIndexSaturatingSub<TupleIndex0>>::Output;
    assert_eq!(<Result4 as TupleIndex>::Idx, 15);
}

#[test]
fn test_saturating_sub_with_saturation() {
    // Test saturation at 0 (when result would be negative)
    type Result1 = <TupleIndex2 as TupleIndexSaturatingSub<TupleIndex10>>::Output;
    assert_eq!(<Result1 as TupleIndex>::Idx, 0);

    type Result2 = <TupleIndex0 as TupleIndexSaturatingSub<TupleIndex5>>::Output;
    assert_eq!(<Result2 as TupleIndex>::Idx, 0);

    type Result3 = <TupleIndex5 as TupleIndexSaturatingSub<TupleIndex50>>::Output;
    assert_eq!(<Result3 as TupleIndex>::Idx, 0);

    type Result4 = <TupleIndex0 as TupleIndexSaturatingSub<TupleIndex63>>::Output;
    assert_eq!(<Result4 as TupleIndex>::Idx, 0);
}

#[test]
fn test_saturating_arithmetic_composition() {
    // (10 - 4) - 2 = 4
    type Step1 = <TupleIndex10 as TupleIndexSaturatingSub<TupleIndex4>>::Output;
    type Step2 = <Step1 as TupleIndexSaturatingSub<TupleIndex2>>::Output;
    assert_eq!(<Step2 as TupleIndex>::Idx, 4);

    // (5 - 10) - 3 = 0 (saturates to 0, stays at 0)
    type Step3 = <TupleIndex5 as TupleIndexSaturatingSub<TupleIndex10>>::Output;
    type Step4 = <Step3 as TupleIndexSaturatingSub<TupleIndex3>>::Output;
    assert_eq!(<Step4 as TupleIndex>::Idx, 0);

    // (20 - 5) - 10 = 5
    type Step5 = <TupleIndex20 as TupleIndexSaturatingSub<TupleIndex5>>::Output;
    type Step6 = <Step5 as TupleIndexSaturatingSub<TupleIndex10>>::Output;
    assert_eq!(<Step6 as TupleIndex>::Idx, 5);
}

#[test]
fn test_saturating_vs_regular_sub() {
    // When no saturation occurs, both should give the same result
    type Regular = <TupleIndex10 as TupleIndexSub<TupleIndex4>>::Output;
    type Saturating = <TupleIndex10 as TupleIndexSaturatingSub<TupleIndex4>>::Output;
    assert_eq!(
        <Regular as TupleIndex>::Idx,
        <Saturating as TupleIndex>::Idx
    );

    type Regular2 = <TupleIndex20 as TupleIndexSub<TupleIndex15>>::Output;
    type Saturating2 = <TupleIndex20 as TupleIndexSaturatingSub<TupleIndex15>>::Output;
    assert_eq!(
        <Regular2 as TupleIndex>::Idx,
        <Saturating2 as TupleIndex>::Idx
    );
}

#[test]
fn test_saturating_sub_with_tuple_access() {
    let tuple = (0u8, 1u16, 2u32, 3u64, 4i8, 5i16, 6i32, 7i64);

    // Access at index 7 - 2 = 5
    type Idx = <TupleIndex7 as TupleIndexSaturatingSub<TupleIndex2>>::Output;
    let value: &i16 = tuple.get::<Idx>();
    assert_eq!(*value, 5i16);

    // Access at index 0 (saturated from 2 - 10)
    type Idx2 = <TupleIndex2 as TupleIndexSaturatingSub<TupleIndex10>>::Output;
    let value2: &u8 = tuple.get::<Idx2>();
    assert_eq!(*value2, 0u8);

    // Access at index 3 (5 - 2)
    type Idx3 = <TupleIndex5 as TupleIndexSaturatingSub<TupleIndex2>>::Output;
    let value3: &u64 = tuple.get::<Idx3>();
    assert_eq!(*value3, 3u64);
}
