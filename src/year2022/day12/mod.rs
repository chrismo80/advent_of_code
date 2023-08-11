pub fn solve() -> (usize, usize)
{
    let input = include_str!("input.txt");

    let result1 = 0;
    let result2 = 0;

    println!("12\t{result1}\t{result2}");

    (result1, result2)
}

#[cfg(test)]
mod tests
{
    #[test]
    fn solve()
    {
        assert_eq!(super::solve(), (0, 0));
    }
}
