use std::collections::*;

pub fn solve()
{
    let input = include_str!("test.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let result1 = input
        .iter()
        .map(|line| line.chunks(2).into_iter())
        .map(|chunks| {
            (
                HashSet::from_iter(chunks.clone().map(|chunk| chunk[0])),
                HashSet::from_iter(chunks.clone().map(|chunk| chunk[1])),
            )
        })
        .collect::<Vec<(HashSet<char>, HashSet<char>)>>()
        .len();

    let result2 = 0;

    println!("3\t{result1}\t{result2}");
}
