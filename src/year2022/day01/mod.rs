use crate::extensions::input_parser::*;

pub fn solve() -> (i64, i64)
{
    let input = include_str!("input.txt").to_vec_of_vec::<i64>("\n\n", "\n");

    let mut data: Vec<i64> = input.iter().map(|cals| cals.iter().sum()).collect();

    data.sort();

    let result1 = *data.last().unwrap();
    let result2 = data.iter().rev().take(3).sum();

    println!("1\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

#[test]
fn test()
{
    assert_eq!(solve(), (72602, 207410));
}
