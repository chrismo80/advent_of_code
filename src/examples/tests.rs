pub fn smaller_than(a: i32, b: i32) -> bool
{
    a < b
}

pub fn prints_and_returns_10(a: i32) -> i32
{
    println!("I got the value {a}");
    10
}

#[cfg(test)] // only compiles when running tests (cargo test)
mod tests
{
    use super::*; // import parent module items

    #[test]
    fn is_smaller_than()
    {
        for i in 0..10 {
            assert!(smaller_than(i, 10));
        }
    }

    #[test]
    fn is_not_smaller_than()
    {
        assert!(!smaller_than(8, 6));
    }

    #[test]
    fn this_test_will_pass()
    {
        assert_eq!(prints_and_returns_10(4), 10);
    }

    #[test]
    #[should_panic = "values don't match"]
    fn this_test_will_fail()
    {
        
        assert_eq!(prints_and_returns_10(8), 5, "values don't match");
    }
}
