use std::collections::HashSet;

pub fn solve() -> (usize, usize)
{
    let input = include_str!("input.txt").split("\n\n").collect::<Vec<&str>>();

    let result1 = input
        .iter()
        .map(|s| s.replace('\n', "").chars().collect::<HashSet<char>>().len())
        .sum();

    let result2 = input
        .iter()
        .map(|s| {
            s.split('\n')
                .map(|line| line.chars().collect::<HashSet<char>>())
                .fold(('a'..='z').collect::<HashSet<char>>(), |last, current| {
                    last.intersection(&current).copied().collect()
                })
                .len()
        })
        .sum();

    println!("6\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

#[test]
fn test()
{
    assert_eq!(solve(), (6878, 3464));
}
