pub fn solve() -> u64
{
    let (mut code, row, col, mut diagonal, mut count) = (20151125, 3010, 3019, 1, 1);

    while row + col > 1 + diagonal {
        diagonal += 1;
        count += diagonal;
    }

    while count > row {
        count -= 1;
        code = code * 252533 % 33554393;
    }

    println!("25\t{code:<20}\t");

    code
}

#[cfg(test)]
mod tests
{
    #[test]
    fn solve()
    {
        assert_eq!(super::solve(), 8997277);
    }
}
