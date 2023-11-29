use crate::extensions::converter::*;

pub fn solve() -> (i64, i64)
{
    let input = include_str!("input.txt").to_vec_of_vec::<i64>("\n\n", "\n");

    let result1 = 1;
    let result2 = 0;

    println!("1\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

#[test]
fn test()
{
    assert_eq!(solve(), (0, 0));
}
