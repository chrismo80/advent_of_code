use std::collections::*;

pub fn solve() -> (usize, usize)
{
    let input = include_str!("input.txt").chars().collect::<Vec<char>>();

    let find_marker = |size: usize, stream: &Vec<char>| {
        stream
            .windows(size)
            .position(|w| w.iter().copied().collect::<HashSet<char>>().len() == size)
            .unwrap_or(stream.len())
            + size
    };

    let result1 = find_marker(4, &input);
    let result2 = find_marker(14, &input);

    println!("6\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

#[test]
fn test()
{
    assert_eq!(solve(), (1238, 3037));
}
