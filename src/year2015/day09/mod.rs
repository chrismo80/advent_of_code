pub fn solve() -> (usize, usize)
{
    let input: Vec<&str> = include_str!("test.txt").lines().collect();

    let result1 = 0;
    let result2 = 0;

    println!("09\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

#[cfg(test)]
mod tests
{
    #[test]
    fn solve()
    {
        assert_eq!(super::solve(), (141, 736));
    }
}
