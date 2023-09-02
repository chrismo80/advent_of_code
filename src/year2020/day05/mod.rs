pub fn solve() -> (i64, i64)
{
    let input = include_str!("input.txt").lines();

    let mut seat_ids: Vec<i64> = input
        .map(|code| 8 * position(&code[..7], 'B', 128) + position(&code[7..], 'R', 8))
        .collect();

    seat_ids.sort();

    let result1 = *seat_ids.iter().max().unwrap();
    let result2 = seat_ids.as_slice().windows(2).find(|&seats| seats[1] - seats[0] > 1).unwrap()[0] + 1;

    println!("5\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

fn position(code: &str, direction: char, end: i64) -> i64
{
    code.chars()
        .fold((0, end), |(min, max), current| match current == direction {
            true => (min + ((max - min) / 2), max),
            false => (min, min + ((max - min) / 2)),
        })
        .0
}

#[test]
fn test()
{
    assert_eq!(solve(), (885, 623));
}
