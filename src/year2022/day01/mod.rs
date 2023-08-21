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

    println!("1\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

#[test]
fn test()
{
    assert_eq!(solve(), (72602, 207410));
}
