pub fn solve() -> (i64, usize)
{
    let input = include_str!("input.txt").chars().enumerate();

    let result = input.fold((0, 0), |(mut result1, mut result2), (index, value)| {
        result1 += if value == '(' { 1 } else { -1 };
        result2 = if result1 < 0 && result2 == 0 { index + 1 } else { result2 };
        (result1, result2)
    });

    println!("1\t{:<20}\t{:<20}", result.0, result.1);

    (result.0, result.1)
}

#[test]
fn test()
{
    assert_eq!(solve(), (138, 1771));
}
