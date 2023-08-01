pub fn solve()
{
    let input = include_str!("input.txt");

    let mut data: Vec<i32> = input
        .split("\n\n")
        .map(|elves| elves.lines().map(|cal| cal.parse::<i32>().unwrap()).sum())
        .collect();

    data.sort();

    let result1 = data.last().unwrap();
    let result2 = data.iter().rev().take(3).sum::<i32>();

    println!("Part 1: {result1}, Part 2: {result2}");
}
