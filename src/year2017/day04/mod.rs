use crate::extensions::input_parser::*;
use iter_tools::Itertools;
use std::collections::HashSet;

pub fn solve() -> (usize, usize)
{
    let input = include_str!("input.txt").to_vec_of_vec::<String>("\n", " ");

    let result1 = input
        .iter()
        .filter(|words| words.len() == words.iter().collect::<HashSet<_>>().len())
        .count();

    let result2 = input
        .iter()
        .map(|words| {
            words
                .iter()
                .map(|word| word.chars().sorted().collect::<String>())
                .collect::<Vec<String>>()
        })
        .filter(|words| words.len() == words.iter().collect::<HashSet<_>>().len())
        .count();

    println!("4\t{:<20}\t{:<20}", result1, result2);

    (result1, result2)
}

#[test]
fn test()
{
    assert_eq!(solve(), (455, 186));
}
