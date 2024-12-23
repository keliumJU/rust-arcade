/// ### Documentation test
/// ```
/// let result = basic_math::add(2,3);
/// assert_eq!(result, 5);
/// ```

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn add_works() {
    assert_eq!(add(1,2), 3);
    assert_eq!(add(10, 12), 22);
    assert_eq!(add(5, -2), 3);
}

#[test]
#[should_panic]
fn add_fails() {
    assert_eq!(add(1,1), 3);
}

#[test]
#[ignore]
fn add_negatives() {
    assert_eq!(add(-1,-2), -3)
}
