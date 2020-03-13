use testing;

#[test]
fn fib_not_zero() {
    assert_ne!(testing::fibonacci(0), 0);
}
