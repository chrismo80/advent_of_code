use super::intcode_computer::*;
use std::collections::*;

pub fn solve() -> (i64, String)
{
    let input = include_str!("input.txt").split(',');

    let memory: HashMap<i64, i64> = input.enumerate().map(|(i, x)| (i as i64, x.parse().unwrap())).collect();

    let result1 = 0;
    let result2 = "0".to_string();

    println!("11\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

#[cfg(test)]
mod tests
{
    #[test]
    fn solve()
    {
        assert_eq!(super::solve(), (2415, "BFPUZUPC".to_string()));
    }
}
