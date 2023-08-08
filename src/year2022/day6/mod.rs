pub fn solve() -> (i32, i32)
{
    let input = include_str!("test.txt");

    for i in (3..input.len()) {
        println!("{}", input[(i - 4)..i]);
    }

    let mut result1 = 0;
    let mut result2 = 0;

    println!("6\t{result1}\t{result2}");

    (result1, result2)
}

#[cfg(test)]
mod tests
{
    #[test]
    fn solve()
    {
        assert_eq!(super::solve(), (7, 19));
    }
}
