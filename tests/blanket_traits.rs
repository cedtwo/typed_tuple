use typed_tuple::TypedTuple;

// Test custom trait with blanket implementation using TypedTuple
// This trait does NOT have a generic const parameter
trait Incrementable {
    fn increment(&mut self);
}

// Blanket implementation for any tuple that contains u32 at index 0
impl<T> Incrementable for T
where
    T: TypedTuple<0, u32>,
{
    fn increment(&mut self) {
        let current: &u32 = self.get();
        self.set(*current + 1);
    }
}

#[test]
fn test_blanket_trait_implementation() {
    // Test with 1-element tuple up to 128 elements
    let mut tuple_1 = (5u32,);
    tuple_1.increment();
    assert_eq!(tuple_1, (6u32,));

    // Test with 2-element tuple up to 128 elements
    let mut tuple_2 = (10u32, "test");
    tuple_2.increment();
    assert_eq!(tuple_2, (11u32, "test"));

    // Test with 3-element tuple up to 128 elements
    let mut tuple_3 = (100u32, 200u64, true);
    tuple_3.increment();
    assert_eq!(tuple_3, (101u32, 200u64, true));

    // Can still increment multiple times
    tuple_3.increment();
    tuple_3.increment();
    assert_eq!(tuple_3, (103u32, 200u64, true));
}

// Another example: trait for accessing element at any index
trait ElementAt<const INDEX: usize, T> {
    fn get_at(&self) -> &T;
    fn set_at(&mut self, value: T);
}

// Blanket implementation for any tuple up to 128 elements that has T at the specified INDEX
impl<const INDEX: usize, M, T> ElementAt<INDEX, T> for M
where
    M: TypedTuple<INDEX, T>,
{
    fn get_at(&self) -> &T {
        self.get()
    }

    fn set_at(&mut self, value: T) {
        self.set(value);
    }
}

#[test]
#[allow(clippy::float_cmp)]
fn test_element_at_trait() {
    let mut tuple_a = (42u32,);
    assert_eq!(*ElementAt::<0, u32>::get_at(&tuple_a), 42u32);
    ElementAt::<0, u32>::set_at(&mut tuple_a, 100u32);
    assert_eq!(tuple_a, (100u32,));

    let mut tuple_b = (10u32, "hello", 3.5);
    assert_eq!(*ElementAt::<0, u32>::get_at(&tuple_b), 10u32);
    ElementAt::<0, u32>::set_at(&mut tuple_b, 99u32);
    assert_eq!(tuple_b, (99u32, "hello", 3.5));

    // Access element at index 1
    assert_eq!(*ElementAt::<1, &str>::get_at(&tuple_b), "hello");
    ElementAt::<1, &str>::set_at(&mut tuple_b, "changed");
    assert_eq!(tuple_b, (99u32, "changed", 3.5));

    // Access element at index 2
    assert_eq!(*ElementAt::<2, f64>::get_at(&tuple_b), 3.5);
    ElementAt::<2, f64>::set_at(&mut tuple_b, 2.71);
    assert_eq!(tuple_b, (99u32, "changed", 2.71));
}
