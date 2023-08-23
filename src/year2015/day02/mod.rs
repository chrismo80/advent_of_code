use iter_tools::Itertools;

pub fn solve() -> (i64, i64)
{
    let input = include_str!("input.txt")
        .lines()
        .map(|l| l.split('x').map(|n| n.parse::<i64>().unwrap()).sorted().collect::<Vec<i64>>());

    let result2 = input.clone().map(|b| 2 * b[0] + 2 * b[1] + b[0] * b[1] * b[2]).sum();
    let result1 = input.map(|b| 3 * b[0] * b[1] + 2 * b[1] * b[2] + 2 * b[2] * b[0]).sum();

    println!("2\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

#[test]
fn test()
{
    assert_eq!(solve(), (1588178, 3783758));
}
