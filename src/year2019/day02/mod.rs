pub fn solve() -> (i32, i32)
{
    let input = include_str!("input.txt");

    let result1 = 0;
    let result2 = 0;

    println!("2\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

#[cfg(test)]
mod tests
{
    #[test]
    fn solve()
    {
        assert_eq!(super::solve(), (5534943, 7603));
    }
}
