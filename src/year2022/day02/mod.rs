pub fn solve() -> (i32, i32)
{
    let input = include_str!("input.txt").lines();

    let data: Vec<(i32, i32)> = input
        .map(|line| {
            (
                line.chars().next().unwrap() as i32 - 'A' as i32,
                line.chars().nth(2).unwrap() as i32 - 'X' as i32,
            )
        })
        .collect();

    let result1: i32 = data.iter().map(|x| ((4 + x.1 - x.0) % 3 * 3) + x.1 + 1).sum();

    let result2: i32 = data
        .iter()
        .map(|x| (x.0, (x.1 + 2 + x.0) % 3))
        .map(|x| ((4 + x.1 - x.0) % 3 * 3) + x.1 + 1)
        .sum();

    println!("2\t{result1:<15}\t{result2:<15}");

    (result1, result2)
}

#[cfg(test)]
mod tests
{
    #[test]
    fn solve()
    {
        assert_eq!(super::solve(), (15523, 15702));
    }
}
