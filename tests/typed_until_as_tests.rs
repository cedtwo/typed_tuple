//! Tests for NthIndexedUntil and NthIndexedAs traits
//!
//! These tests verify that the traits are correctly implemented for tuples.

use typed_tuple::prelude::*;

#[test]
fn test_nth_indexed_until_concrete() {
    // Verify that concrete tuples implement NthIndexedUntil
    // These are compile-time checks via trait bounds

    fn _assert_nth_indexed_until<T: NthIndexedUntil<TupleIndex2>>(_val: &T) {}

    let tuple1 = (1u8, 2u16, 3u32, 4u64);
    _assert_nth_indexed_until(&tuple1);

    let tuple2 = (5u8, 6u16, 7u32);
    _assert_nth_indexed_until(&tuple2);
}

#[test]
fn test_nth_indexed_as_matching_types() {
    // Test that tuples with matching types implement NthIndexedAs

    type T1 = (u8, u16, u32, u64);
    type T2 = (u8, u16, u32, String);

    fn _assert_match<TT: NthIndexedAs<TupleIndex2, T2>>(_val: &TT) {}

    let tuple1: T1 = (1, 2, 3, 4);
    _assert_match(&tuple1);
}

#[test]
fn test_nth_indexed_as_self_matching() {
    // A tuple should match itself

    type T = (u8, u16, u32, u64, i8);

    fn _assert_self_match<TT: NthIndexedAs<TupleIndex3, T>>(_val: &TT) {}

    let tuple: T = (1, 2, 3, 4, -5);
    _assert_self_match(&tuple);
}

#[test]
fn test_nth_indexed_as_different_lengths() {
    // Different length tuples can match up to a certain index

    type T1 = (u8, u16, u32, u64);
    type T2 = (u8, u16);

    fn _assert_partial_match<TT: NthIndexedAs<TupleIndex1, T2>>(_val: &TT) {}

    let tuple1: T1 = (1, 2, 3, 4);
    _assert_partial_match(&tuple1);
}

#[test]
fn test_nth_indexed_as_index0() {
    // Test matching at index 0 (first element only)

    type T1 = (u8, String);
    type T2 = (u8, i32, bool);

    fn _assert_first_match<TT: NthIndexedAs<TupleIndex0, T2>>(_val: &TT) {}

    let tuple1: T1 = (42, "hello".to_string());
    _assert_first_match(&tuple1);
}
