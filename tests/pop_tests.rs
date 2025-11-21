use typed_tuple::TypedTuple;

#[test]
fn test_pop_first_element() {
    let tuple = ("hello", 42, 2.5);
    let (val, rest): (&str, _) = tuple.pop();
    assert_eq!(val, "hello");
    assert_eq!(rest, (42, 2.5));
}

#[test]
fn test_pop_middle_element() {
    let tuple = ("hello", 42, 2.5);
    let (val, rest) = TypedTuple::<1, _>::pop(tuple);
    assert_eq!(val, 42);
    assert_eq!(rest, ("hello", 2.5));
}

#[test]
fn test_pop_last_element() {
    let tuple = ("hello", 42, 2.5);
    let (val, rest) = TypedTuple::<2, _>::pop(tuple);
    assert_eq!(val, 2.5);
    assert_eq!(rest, ("hello", 42));
}

#[test]
fn test_pop_single_element_tuple() {
    let tuple = (42,);
    let (val, rest) = tuple.pop();
    assert_eq!(val, 42);
    assert_eq!(rest, ());
}

#[test]
fn test_pop_two_element_tuple() {
    let tuple = (1, 2);
    let (val, rest) = TypedTuple::<0, _>::pop(tuple);
    assert_eq!(val, 1);
    assert_eq!(rest, (2,));

    let tuple = (1, 2);
    let (val, rest) = TypedTuple::<1, _>::pop(tuple);
    assert_eq!(val, 2);
    assert_eq!(rest, (1,));
}

#[test]
fn test_pop_large_tuple() {
    let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
    let (val, rest) = TypedTuple::<5, _>::pop(tuple);
    assert_eq!(val, 6);
    assert_eq!(rest, (1, 2, 3, 4, 5, 7, 8, 9, 10));
}
