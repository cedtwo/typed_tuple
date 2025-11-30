//! Tests for the LastIndex functionality.

use typed_tuple::prelude::*;

#[test]
fn test_last_get_single_element() {
    type TupleType = (u32,);
    type LastIdx = <TupleType as LastIndex>::Last;

    let tuple: TupleType = (42,);
    let last: &u32 = tuple.get::<LastIdx>();
    assert_eq!(*last, 42u32);
}

#[test]
fn test_last_get_two_elements() {
    type TupleType = (u8, u16);
    type LastIdx = <TupleType as LastIndex>::Last;

    let tuple: TupleType = (1, 2);
    let last: &u16 = tuple.get::<LastIdx>();
    assert_eq!(*last, 2u16);
}

#[test]
fn test_last_get_five_elements() {
    type TupleType = (u8, u16, u32, u64, i8);
    type LastIdx = <TupleType as LastIndex>::Last;

    let tuple: TupleType = (1, 2, 3, 4, 5);
    let last: &i8 = tuple.get::<LastIdx>();
    assert_eq!(*last, 5i8);
}

#[test]
fn test_last_get_mut() {
    type TupleType = (u8, u16, u32);
    type LastIdx = <TupleType as LastIndex>::Last;

    let mut tuple: TupleType = (1, 2, 3);
    let last: &mut u32 = tuple.get_mut::<LastIdx>();
    *last = 100u32;
    assert_eq!(tuple, (1u8, 2u16, 100u32));
}

#[test]
fn test_last_replace() {
    type TupleType = (String, u32, f64);
    type LastIdx = <TupleType as LastIndex>::Last;

    let mut tuple: TupleType = (String::from("hello"), 42, 3.14);
    let old: f64 = tuple.replace::<LastIdx>(2.71f64);
    assert_eq!(old, 3.14f64);
    assert_eq!(tuple, (String::from("hello"), 42u32, 2.71f64));
}

#[test]
fn test_last_apply() {
    type TupleType = (u8, u16, u32);
    type LastIdx = <TupleType as LastIndex>::Last;

    let mut tuple: TupleType = (1, 2, 3);
    tuple.apply::<LastIdx, _>(|x| *x *= 10);
    assert_eq!(tuple, (1u8, 2u16, 30u32));
}

#[test]
fn test_last_apply_string() {
    type TupleType = (i32, String);
    type LastIdx = <TupleType as LastIndex>::Last;

    let mut tuple: TupleType = (42, "hello".to_string());
    tuple.apply::<LastIdx, _>(|s| *s = s.to_uppercase());
    assert_eq!(tuple, (42, "HELLO".to_string()));
}

#[test]
fn test_last_map() {
    type TupleType = (u8, u16, u32, u64);
    type LastIdx = <TupleType as LastIndex>::Last;

    let tuple: TupleType = (1, 2, 3, 4);
    let result: u64 = tuple.map::<LastIdx, _, _>(|x| x * 2);
    assert_eq!(result, 8u64);
}

#[test]
fn test_last_pop_two_elements() {
    type TupleType = (u8, u16);
    type LastIdx = <TupleType as LastIndex>::Last;

    let tuple: TupleType = (1, 2);
    let (last, rest): (u16, _) = tuple.pop::<LastIdx>();
    assert_eq!(last, 2u16);
    assert_eq!(rest, (1u8,));
}

#[test]
fn test_last_pop_three_elements() {
    type TupleType = (u8, u16, u32);
    type LastIdx = <TupleType as LastIndex>::Last;

    let tuple: TupleType = (1, 2, 3);
    let (last, rest): (u32, _) = tuple.pop::<LastIdx>();
    assert_eq!(last, 3u32);
    assert_eq!(rest, (1u8, 2u16));
}

#[test]
fn test_last_take() {
    type TupleType = (String, String, String);
    type LastIdx = <TupleType as LastIndex>::Last;

    let mut tuple: TupleType = (
        String::from("first"),
        String::from("second"),
        String::from("last"),
    );
    let last: String = tuple.take::<LastIdx>();
    assert_eq!(last, "last");
    assert_eq!(
        tuple,
        (String::from("first"), String::from("second"), String::new())
    );
}

#[test]
fn test_last_split_exclusive() {
    type TupleType = (u8, u16, u32, u64);
    type LastIdx = <TupleType as LastIndex>::Last;

    let tuple: TupleType = (1, 2, 3, 4);
    let (left, element, right) = tuple.split_exclusive::<LastIdx>();
    assert_eq!(left, (1u8, 2u16, 3u32));
    assert_eq!(element, 4u64);
    assert_eq!(right, ());
}

#[test]
fn test_last_split_left() {
    type TupleType = (u8, u16, u32, u64);
    type LastIdx = <TupleType as LastIndex>::Last;

    let tuple: TupleType = (1, 2, 3, 4);
    let (left, right) = tuple.split_left::<LastIdx>();
    assert_eq!(left, (1u8, 2u16, 3u32, 4u64));
    assert_eq!(right, ());
}

#[test]
fn test_last_split_right() {
    type TupleType = (u8, u16, u32, u64);
    type LastIdx = <TupleType as LastIndex>::Last;

    let tuple: TupleType = (1, 2, 3, 4);
    let (left, right) = tuple.split_right::<LastIdx>();
    assert_eq!(left, (1u8, 2u16, 3u32));
    assert_eq!(right, (4u64,));
}

#[test]
fn test_last_with_different_tuple_sizes() {
    // Test that LastIndex adapts to different tuple sizes
    type T1 = (i32,);
    type T2 = (i32, i32);
    type T3 = (i32, i32, i32);
    type T4 = (i32, i32, i32, i32);
    type T5 = (i32, i32, i32, i32, i32);

    let tuple1: T1 = (100,);
    let tuple2: T2 = (1, 200);
    let tuple3: T3 = (1, 2, 300);
    let tuple4: T4 = (1, 2, 3, 400);
    let tuple5: T5 = (1, 2, 3, 4, 500);

    assert_eq!(*tuple1.get::<<T1 as LastIndex>::Last>(), 100);
    assert_eq!(*tuple2.get::<<T2 as LastIndex>::Last>(), 200);
    assert_eq!(*tuple3.get::<<T3 as LastIndex>::Last>(), 300);
    assert_eq!(*tuple4.get::<<T4 as LastIndex>::Last>(), 400);
    assert_eq!(*tuple5.get::<<T5 as LastIndex>::Last>(), 500);
}

#[test]
fn test_last_with_mixed_types() {
    type TupleType1 = (u32, &'static str, f64, bool, char);
    type LastIdx1 = <TupleType1 as LastIndex>::Last;

    let tuple: TupleType1 = (42, "hello", 3.14, true, 'x');
    let last: &char = tuple.get::<LastIdx1>();
    assert_eq!(*last, 'x');

    type TupleType2 = (u8, Vec<i32>);
    type LastIdx2 = <TupleType2 as LastIndex>::Last;

    let tuple2: TupleType2 = (1, vec![1, 2, 3]);
    let last_vec: &Vec<i32> = tuple2.get::<LastIdx2>();
    assert_eq!(last_vec, &vec![1, 2, 3]);
}

#[test]
fn test_last_index_trait() {
    // Verify that LastIndex is properly implemented
    type TupleType = (u8, u16, u32);
    type LastMarker = <TupleType as LastIndex>::Last;

    // The Last associated type should be TupleIndex2 for a 3-element tuple
    let tuple: TupleType = (1, 2, 3);
    let last: &u32 = tuple.get::<LastMarker>();
    assert_eq!(*last, 3u32);
}

#[test]
fn test_last_swap() {
    // LastIndex works with swap when there are duplicate types
    type TupleType = (u32, u32, u32);
    type LastIdx = <TupleType as LastIndex>::Last;

    let mut tuple: TupleType = (1, 2, 3);

    // Swap first and last u32
    tuple.swap::<TupleIndex0, LastIdx>();
    assert_eq!(tuple, (3u32, 2u32, 1u32));
}
