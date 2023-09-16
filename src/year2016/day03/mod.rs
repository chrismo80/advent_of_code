use crate::extensions::converter::Parser;

pub fn solve() -> (usize, usize)
{
    let input = include_str!("input.txt").to_vec_of_vec::<usize>("\n", " ");

    let is_triangle =
        |sides: &[usize]| sides[0] + sides[1] > sides[2] && sides[0] + sides[2] > sides[1] && sides[1] + sides[2] > sides[0];

    let result1 = input.iter().filter(|sides| is_triangle(sides)).count();

    let result2 = input
        .iter()
        .map(|s| s[0])
        .chain(input.iter().map(|s| s[1]))
        .chain(input.iter().map(|s| s[2]))
        .collect::<Vec<usize>>()
        .chunks(3)
        .filter(|sides| is_triangle(sides))
        .count();

    println!("3\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

#[test]
fn test()
{
    assert_eq!(solve(), (993, 1849));
}
