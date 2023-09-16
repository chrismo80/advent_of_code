use crate::extensions::converter::Parser;

pub fn solve() -> (usize, usize)
{
    let input = include_str!("input.txt").to_vec_of_vec_of_vec::<i32>("\n", ",", "-");

    let fully = |min1: i32, max1: i32, min2: i32, max2: i32| min1 <= min2 && max1 >= max2 || min2 <= min1 && max2 >= max1;
    let partially = |min1: i32, max1: i32, min2: i32, max2: i32| min1 <= max2 && max1 >= min2;

    let result1 = input.iter().filter(|l| fully(l[0][0], l[0][1], l[1][0], l[1][1])).count();
    let result2 = input.iter().filter(|l| partially(l[0][0], l[0][1], l[1][0], l[1][1])).count();

    println!("4\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

#[test]
fn test()
{
    assert_eq!(solve(), (556, 876));
}
