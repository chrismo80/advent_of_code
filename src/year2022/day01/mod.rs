pub fn solve() -> (i32, i32)
{
    let input = include_str!("input.txt");

    let mut data: Vec<i32> = input
        .split("\n\n")
        .map(|elves| elves.lines().map(|cal| cal.parse::<i32>().unwrap()).sum())
        .collect();

    data.sort();

    let result1 = *data.last().unwrap();
    let result2 = data.iter().rev().take(3).sum::<i32>();

    println!("1\t{result1}\t\t{result2}");

    (result1, result2)
}

#[cfg(test)]
mod tests
{
    #[test]
    fn solve()
    {
        assert_eq!(super::solve(), (72602, 207410));
    }
}
