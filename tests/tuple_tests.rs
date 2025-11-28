//! Basic tests for TypedTuple functionality.

use typed_tuple::prelude::*;

#[test]
fn test_tuple_1() {
    let mut tuple = (1u8,);

    // Test get
    let val: &u8 = tuple.get();
    assert_eq!(*val, 1u8);

    // Test get_mut
    *tuple.get_mut() = 10u8;
    assert_eq!(tuple, (10u8,));

    // Test replace
    let old = tuple.replace(20u8);
    assert_eq!(old, 10u8);
    assert_eq!(tuple, (20u8,));

    // Test apply
    tuple.apply(|x: &mut u8| *x *= 2);
    assert_eq!(tuple, (40u8,));
}

#[test]
fn test_tuple_2() {
    let mut tuple = (1u8, 2u16);

    // Test get
    let a: &u8 = tuple.get();
    let b: &u16 = tuple.get();
    assert_eq!(*a, 1u8);
    assert_eq!(*b, 2u16);

    // Test get_mut
    *tuple.get_mut() = 10u8;
    *tuple.get_mut() = 20u16;
    assert_eq!(tuple, (10u8, 20u16));

    // Test replace
    tuple.replace(30u8);
    tuple.replace(40u16);
    assert_eq!(tuple, (30u8, 40u16));

    // Test apply
    tuple.apply(|x: &mut u8| *x *= 2);
    tuple.apply(|x: &mut u16| *x *= 2);
    assert_eq!(tuple, (60u8, 80u16));
}

#[test]
fn test_tuple_3() {
    let mut tuple = (1u8, 2u16, 3u32);

    // Test get
    let _: &u8 = tuple.get();
    let _: &u16 = tuple.get();
    let _: &u32 = tuple.get();

    // Test get_mut and set
    *tuple.get_mut() = 10u8;
    *tuple.get_mut() = 20u16;
    *tuple.get_mut() = 30u32;
    assert_eq!(tuple, (10u8, 20u16, 30u32));

    // Test apply
    tuple.apply(|x: &mut u8| *x += 1);
    tuple.apply(|x: &mut u16| *x += 1);
    tuple.apply(|x: &mut u32| *x += 1);
    assert_eq!(tuple, (11u8, 21u16, 31u32));
}

#[test]
fn test_tuple_4() {
    let mut tuple = (1u8, 2u16, 3u32, 4u64);

    *tuple.get_mut() = 10u8;
    *tuple.get_mut() = 20u16;
    *tuple.get_mut() = 30u32;
    *tuple.get_mut() = 40u64;
    assert_eq!(tuple, (10u8, 20u16, 30u32, 40u64));
}

#[test]
fn test_tuple_5() {
    let mut tuple = (1u8, 2u16, 3u32, 4u64, 5i8);

    tuple.replace(10u8);
    tuple.replace(20u16);
    tuple.replace(30u32);
    tuple.replace(40u64);
    tuple.replace(-5i8);
    assert_eq!(tuple, (10u8, 20u16, 30u32, 40u64, -5i8));
}

#[test]
fn test_tuple_6() {
    let mut tuple = (1u8, 2u16, 3u32, 4u64, 5i8, 6i16);

    tuple.apply(|x: &mut u8| *x *= 2);
    tuple.apply(|x: &mut u16| *x *= 2);
    tuple.apply(|x: &mut u32| *x *= 2);
    tuple.apply(|x: &mut u64| *x *= 2);
    tuple.apply(|x: &mut i8| *x *= 2);
    tuple.apply(|x: &mut i16| *x *= 2);
    assert_eq!(tuple, (2u8, 4u16, 6u32, 8u64, 10i8, 12i16));
}

#[test]
fn test_tuple_7() {
    let mut tuple = (1u8, 2u16, 3u32, 4u64, 5i8, 6i16, 7i32);

    let _: &u8 = tuple.get();
    let _: &u16 = tuple.get();
    let _: &u32 = tuple.get();
    let _: &u64 = tuple.get();
    let _: &i8 = tuple.get();
    let _: &i16 = tuple.get();
    let _: &i32 = tuple.get();

    *tuple.get_mut() = 100u8;
    assert_eq!(tuple.0, 100u8);
}

#[test]
fn test_tuple_8() {
    let mut tuple = (1u8, 2u16, 3u32, 4u64, 5i8, 6i16, 7i32, 8i64);

    tuple.replace(10u8);
    tuple.replace(20u16);
    tuple.replace(30u32);
    tuple.replace(40u64);
    tuple.replace(50i8);
    tuple.replace(60i16);
    tuple.replace(70i32);
    tuple.replace(80i64);
    assert_eq!(
        tuple,
        (10u8, 20u16, 30u32, 40u64, 50i8, 60i16, 70i32, 80i64)
    );
}

#[test]
#[allow(clippy::float_cmp)]
fn test_tuple_9() {
    let mut tuple = (1u8, 2u16, 3u32, 4u64, 5i8, 6i16, 7i32, 8i64, 9f32);

    *tuple.get_mut() = 1.5f32;
    let val: &f32 = tuple.get();
    assert_eq!(*val, 1.5f32);
}

#[test]
fn test_tuple_10() {
    let mut tuple = (1u8, 2u16, 3u32, 4u64, 5i8, 6i16, 7i32, 8i64, 9f32, 10f64);

    tuple.apply(|x: &mut u8| *x += 1);
    tuple.apply(|x: &mut u16| *x += 1);
    tuple.apply(|x: &mut u32| *x += 1);
    tuple.apply(|x: &mut u64| *x += 1);
    tuple.apply(|x: &mut i8| *x += 1);
    tuple.apply(|x: &mut i16| *x += 1);
    tuple.apply(|x: &mut i32| *x += 1);
    tuple.apply(|x: &mut i64| *x += 1);
    tuple.apply(|x: &mut f32| *x += 1.0);
    tuple.apply(|x: &mut f64| *x += 1.0);
    assert_eq!(
        tuple,
        (
            2u8, 3u16, 4u32, 5u64, 6i8, 7i16, 8i32, 9i64, 10.0f32, 11.0f64
        )
    );
}

// Additional edge case tests
#[test]
#[allow(clippy::float_cmp)]
fn test_mixed_types() {
    let mut tuple = (42usize, "hello", 3.5f32, true, 'x');

    // Test get with mixed types
    let num: &usize = tuple.get();
    let text: &&str = tuple.get();
    let float: &f32 = tuple.get();
    let boolean: &bool = tuple.get();
    let character: &char = tuple.get();

    assert_eq!(*num, 42);
    assert_eq!(*text, "hello");
    assert_eq!(*float, 3.5);
    assert!(*boolean);
    assert_eq!(*character, 'x');

    // Test mutations
    *tuple.get_mut() = 100usize;
    *tuple.get_mut() = "world";
    *tuple.get_mut() = 2.71f32;
    *tuple.get_mut() = false;
    *tuple.get_mut() = 'y';

    assert_eq!(tuple, (100, "world", 2.71, false, 'y'));
}

#[test]
fn test_const_index_access() {
    let mut tuple = (1u8, 2u16, 3u32);

    // Access by const index
    let first = tuple.get::<TupleIndex0>();
    let second = tuple.get::<TupleIndex1>();
    let third = tuple.get::<TupleIndex2>();

    assert_eq!(*first, 1u8);
    assert_eq!(*second, 2u16);
    assert_eq!(*third, 3u32);

    // Mutate by const index
    *tuple.get_mut::<TupleIndex0>() = 10u8;
    *tuple.get_mut::<TupleIndex1>() = 20u16;
    *tuple.get_mut::<TupleIndex2>() = 30u32;

    assert_eq!(tuple, (10u8, 20u16, 30u32));
}

#[test]
fn test_apply_with_string() {
    let mut tuple = ("hello".to_string(), 42u32);

    tuple.apply(|s: &mut String| *s = s.to_uppercase());
    tuple.apply(|n: &mut u32| *n *= 2);

    assert_eq!(tuple, ("HELLO".to_string(), 84u32));
}
