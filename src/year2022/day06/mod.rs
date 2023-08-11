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

    println!("6\t{result1}\t\t{result2}");

    (result1, result2)
}

#[cfg(test)]
mod tests
{
    #[test]
    fn solve()
    {
        assert_eq!(super::solve(), (1238, 3037));
    }
}
