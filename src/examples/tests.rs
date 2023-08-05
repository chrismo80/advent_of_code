pub fn prints_and_returns_10(a: i32) -> i32
{
    println!("I got the value {}", a);
    10
}

#[cfg(test)] // only compiles when running tests (cargo test)
mod tests
{
    use super::*; // import parent module items

    #[test]
    fn this_test_will_pass()
    {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    #[should_panic = "values don't match"]
    fn this_test_will_fail()
    {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value, "values don't match");
    }
}
