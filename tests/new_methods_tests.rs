use typed_tuple::TypedTuple;

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
    let old = TypedTuple::<1, _>::replace(&mut tuple, 99);
    assert_eq!(old, 42);
    assert_eq!(tuple, ("hello", 99, 2.5));
}

#[test]
fn test_replace_last() {
    let mut tuple = (1u8, 2u16, 3u32);
    let old = TypedTuple::<2, _>::replace(&mut tuple, 100u32);
    assert_eq!(old, 3u32);
    assert_eq!(tuple, (1u8, 2u16, 100u32));
}

#[test]
fn test_swap_first_and_last() {
    let mut tuple = (1u32, 2u16, 3u32);
    TypedTuple::<0, u32>::swap::<2>(&mut tuple);
    assert_eq!(tuple, (3u32, 2u16, 1u32));
}

#[test]
fn test_swap_no_op_same_index() {
    let mut tuple = (1u8, 2u16, 3u32);
    TypedTuple::<1, u16>::swap::<1>(&mut tuple);
    assert_eq!(tuple, (1u8, 2u16, 3u32));
}

#[test]
fn test_swap_strings() {
    let mut tuple = ("a", 42, "b", 2.5, "c");
    TypedTuple::<0, &str>::swap::<2>(&mut tuple);
    assert_eq!(tuple, ("b", 42, "a", 2.5, "c"));

    TypedTuple::<2, &str>::swap::<4>(&mut tuple);
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
    let v = TypedTuple::<1, Vec<i32>>::take(&mut tuple);
    assert_eq!(v, vec![1, 2, 3]);
    assert_eq!(tuple, (1u8, Vec::new(), 'x'));
}

#[test]
fn test_take_number() {
    let mut tuple = (1u8, 2u16, 3u32);
    let val = TypedTuple::<2, u32>::take(&mut tuple);
    assert_eq!(val, 3u32);
    assert_eq!(tuple, (1u8, 2u16, 0u32));
}

#[test]
fn test_split_at_first() {
    let tuple = ("hello", 42, 2.5, 'x', true);
    let (left, right) = TypedTuple::<0, &str>::split_at(tuple);
    assert_eq!(left, ("hello",));
    assert_eq!(right, (42, 2.5, 'x', true));
}

#[test]
fn test_split_at_middle() {
    let tuple = (1u8, 2u16, 3u32, 4u64, 5i8);
    let (left, right) = TypedTuple::<2, u32>::split_at(tuple);
    assert_eq!(left, (1u8, 2u16, 3u32));
    assert_eq!(right, (4u64, 5i8));
}

#[test]
fn test_split_at_last() {
    let tuple = ("a", 1, "b", 2, "c");
    let (left, right) = TypedTuple::<4, &str>::split_at(tuple);
    assert_eq!(left, ("a", 1, "b", 2, "c"));
    assert_eq!(right, ());
}

#[test]
fn test_split_two_element() {
    let tuple = (1u8, 2u16);
    let (left, right) = TypedTuple::<0, u8>::split_at(tuple);
    assert_eq!(left, (1u8,));
    assert_eq!(right, (2u16,));

    let tuple = (1u8, 2u16);
    let (left, right) = TypedTuple::<1, u16>::split_at(tuple);
    assert_eq!(left, (1u8, 2u16));
    assert_eq!(right, ());
}

#[test]
fn test_combined_operations() {
    let mut tuple = (1u8, 2u16, 3u32, 4u8, 5u16);

    // Swap elements with same type
    TypedTuple::<0, u8>::swap::<3>(&mut tuple);
    assert_eq!(tuple, (4u8, 2u16, 3u32, 1u8, 5u16));

    // Replace an element
    let old = TypedTuple::<2, u32>::replace(&mut tuple, 99u32);
    assert_eq!(old, 3u32);
    assert_eq!(tuple, (4u8, 2u16, 99u32, 1u8, 5u16));

    // Split it
    let (left, right) = TypedTuple::<2, u32>::split_at(tuple);
    assert_eq!(left, (4u8, 2u16, 99u32));
    assert_eq!(right, (1u8, 5u16));
}
