//! Tests for new methods including replace, swap, split, and take.

use typed_tuple::prelude::*;

#[test]
fn test_replace_first() {
    let mut tuple = ("hello", 42, 2.5);
    let old: &str = tuple.replace("world");
    assert_eq!(old, "hello");
    assert_eq!(tuple, ("world", 42, 2.5));
}

#[test]
fn test_replace_middle() {
    let mut tuple = ("hello", 42, 2.5);
    let old = tuple.replace::<TupleIndex1>(99);
    assert_eq!(old, 42);
    assert_eq!(tuple, ("hello", 99, 2.5));
}

#[test]
fn test_replace_last() {
    let mut tuple = (1u8, 2u16, 3u32);
    let old = tuple.replace::<TupleIndex2>(100u32);
    assert_eq!(old, 3u32);
    assert_eq!(tuple, (1u8, 2u16, 100u32));
}

#[test]
fn test_swap_first_and_last() {
    let mut tuple = (1u32, 2u16, 3u32);
    tuple.swap::<TupleIndex0, TupleIndex2>();
    assert_eq!(tuple, (3u32, 2u16, 1u32));
}

#[test]
fn test_swap_no_op_same_index() {
    let mut tuple = (1u8, 2u16, 3u32);
    tuple.swap::<TupleIndex1, TupleIndex1>();
    assert_eq!(tuple, (1u8, 2u16, 3u32));
}

#[test]
fn test_swap_strings() {
    let mut tuple = ("a", 42, "b", 2.5, "c");
    tuple.swap::<TupleIndex0, TupleIndex2>();
    assert_eq!(tuple, ("b", 42, "a", 2.5, "c"));

    tuple.swap::<TupleIndex2, TupleIndex4>();
    assert_eq!(tuple, ("b", 42, "c", 2.5, "a"));
}

#[test]
fn test_take_string() {
    let mut tuple = (String::from("hello"), 42, 2.5);
    let s: String = tuple.take();
    assert_eq!(s, "hello");
    assert_eq!(tuple, (String::new(), 42, 2.5));
}

#[test]
fn test_take_vec() {
    let mut tuple = (1u8, vec![1, 2, 3], 'x');
    let v = tuple.take::<TupleIndex1>();
    assert_eq!(v, vec![1, 2, 3]);
    assert_eq!(tuple, (1u8, Vec::new(), 'x'));
}

#[test]
fn test_take_number() {
    let mut tuple = (1u8, 2u16, 3u32);
    let val = tuple.take::<TupleIndex2>();
    assert_eq!(val, 3u32);
    assert_eq!(tuple, (1u8, 2u16, 0u32));
}

#[test]
fn test_split_at_first() {
    let tuple = ("hello", 42, 2.5, 'x', true);
    let (left, right) = tuple.split_left::<TupleIndex0>();
    assert_eq!(left, ("hello",));
    assert_eq!(right, (42, 2.5, 'x', true));
}

#[test]
fn test_split_at_middle() {
    let tuple = (1u8, 2u16, 3u32, 4u64, 5i8);
    let (left, right) = tuple.split_left::<TupleIndex2>();
    assert_eq!(left, (1u8, 2u16, 3u32));
    assert_eq!(right, (4u64, 5i8));
}

#[test]
fn test_split_at_last() {
    let tuple = ("a", 1, "b", 2, "c");
    let (left, right) = tuple.split_left::<TupleIndex4>();
    assert_eq!(left, ("a", 1, "b", 2, "c"));
    assert_eq!(right, ());
}

#[test]
fn test_split_two_element() {
    let tuple = (1u8, 2u16);
    let (left, right) = tuple.split_left::<TupleIndex0>();
    assert_eq!(left, (1u8,));
    assert_eq!(right, (2u16,));

    let tuple = (1u8, 2u16);
    let (left, right) = tuple.split_left::<TupleIndex1>();
    assert_eq!(left, (1u8, 2u16));
    assert_eq!(right, ());
}

#[test]
fn test_combined_operations() {
    let mut tuple = (1u8, 2u16, 3u32, 4u8, 5u16);

    // Swap elements with same type
    tuple.swap::<TupleIndex0, TupleIndex3>();
    assert_eq!(tuple, (4u8, 2u16, 3u32, 1u8, 5u16));

    // Replace an element
    let old = tuple.replace::<TupleIndex2>(99u32);
    assert_eq!(old, 3u32);
    assert_eq!(tuple, (4u8, 2u16, 99u32, 1u8, 5u16));

    // Split it
    let (left, right) = tuple.split_left::<TupleIndex2>();
    assert_eq!(left, (4u8, 2u16, 99u32));
    assert_eq!(right, (1u8, 5u16));
}
