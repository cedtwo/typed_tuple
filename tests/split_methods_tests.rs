//! Tests for split methods (split_exclusive, split_left, split_right,
//! split_inclusive).

use typed_tuple::prelude::*;

#[test]
fn test_split_exclusive_basic() {
    let tuple = (1u8, 2u16, 3u32, 4u64, 5i8);
    let (left, element, right) = tuple.split_exclusive::<typenum::U2>();
    assert_eq!(left, (1u8, 2u16));
    assert_eq!(element, 3u32);
    assert_eq!(right, (4u64, 5i8));
}

#[test]
fn test_split_exclusive_first() {
    let tuple = (1u8, 2u16, 3u32);
    let (left, element, right) = tuple.split_exclusive::<typenum::U0>();
    assert_eq!(left, ());
    assert_eq!(element, 1u8);
    assert_eq!(right, (2u16, 3u32));
}

#[test]
fn test_split_exclusive_last() {
    let tuple = (1u8, 2u16, 3u32);
    let (left, element, right) = tuple.split_exclusive::<typenum::U2>();
    assert_eq!(left, (1u8, 2u16));
    assert_eq!(element, 3u32);
    assert_eq!(right, ());
}

#[test]
fn test_split_left_basic() {
    let tuple = (1u8, 2u16, 3u32, 4u64, 5i8);
    let (left, right) = tuple.split_left::<typenum::U2>();
    assert_eq!(left, (1u8, 2u16, 3u32));
    assert_eq!(right, (4u64, 5i8));
}

#[test]
fn test_split_left_first() {
    let tuple = (1u8, 2u16, 3u32);
    let (left, right) = tuple.split_left::<typenum::U0>();
    assert_eq!(left, (1u8,));
    assert_eq!(right, (2u16, 3u32));
}

#[test]
fn test_split_left_last() {
    let tuple = (1u8, 2u16, 3u32);
    let (left, right) = tuple.split_left::<typenum::U2>();
    assert_eq!(left, (1u8, 2u16, 3u32));
    assert_eq!(right, ());
}

#[test]
fn test_split_right_basic() {
    let tuple = (1u8, 2u16, 3u32, 4u64, 5i8);
    let (left, right) = tuple.split_right::<typenum::U2>();
    assert_eq!(left, (1u8, 2u16));
    assert_eq!(right, (3u32, 4u64, 5i8));
}

#[test]
fn test_split_right_first() {
    let tuple = (1u8, 2u16, 3u32);
    let (left, right) = tuple.split_right::<typenum::U0>();
    assert_eq!(left, ());
    assert_eq!(right, (1u8, 2u16, 3u32));
}

#[test]
fn test_split_right_last() {
    let tuple = (1u8, 2u16, 3u32);
    let (left, right) = tuple.split_right::<typenum::U2>();
    assert_eq!(left, (1u8, 2u16));
    assert_eq!(right, (3u32,));
}

#[test]
fn test_all_split_methods_together() {
    // Test that all four methods work correctly on the same tuple structure
    let tuple1 = (1u8, 2u16, 3u32, 4u64, 5i8);
    let (left_ex, elem, right_ex) = tuple1.split_exclusive::<typenum::U2>();
    assert_eq!(left_ex, (1u8, 2u16));
    assert_eq!(elem, 3u32);
    assert_eq!(right_ex, (4u64, 5i8));

    let tuple2 = (1u8, 2u16, 3u32, 4u64, 5i8);
    let (left_l, right_l) = tuple2.split_left::<typenum::U2>();
    assert_eq!(left_l, (1u8, 2u16, 3u32));
    assert_eq!(right_l, (4u64, 5i8));

    let tuple3 = (1u8, 2u16, 3u32, 4u64, 5i8);
    let (left_r, right_r) = tuple3.split_right::<typenum::U2>();
    assert_eq!(left_r, (1u8, 2u16));
    assert_eq!(right_r, (3u32, 4u64, 5i8));
}

#[test]
fn test_split_single_element() {
    let tuple = (42u32,);

    let (left, elem, right) = tuple.split_exclusive::<typenum::U0>();
    assert_eq!(left, ());
    assert_eq!(elem, 42u32);
    assert_eq!(right, ());

    let tuple = (42u32,);
    let (left, right) = tuple.split_left::<typenum::U0>();
    assert_eq!(left, (42u32,));
    assert_eq!(right, ());

    let tuple = (42u32,);
    let (left, right) = tuple.split_right::<typenum::U0>();
    assert_eq!(left, ());
    assert_eq!(right, (42u32,));
}
