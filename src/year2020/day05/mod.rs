pub fn solve() -> (i64, i64)
{
    let input = include_str!("input.txt").lines();

    let mut seat_ids: Vec<i64> = input
        .map(|code| 8 * position(&code[..7], 'B', 128) + position(&code[7..], 'R', 8))
        .collect();

    seat_ids.sort();

    let result1 = *seat_ids.iter().max().unwrap();
    let result2 = first_empty(&seat_ids);

    println!("5\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

fn position(code: &str, direction: char, end: i64) -> i64
{
    code.chars()
        .fold((0, end), |(min, max), value| {
            let mid = min + ((max - min) / 2);
            match value == direction {
                true => (mid, max),
                false => (min, mid),
            }
        })
        .0
}

fn first_empty(seat_ids: &[i64]) -> i64
{
    seat_ids.iter().fold(seat_ids[0], |prev, &seat_id| match seat_id - prev {
        1 => seat_id,
        _ => prev,
    }) + 1
}

#[test]
fn test()
{
    assert_eq!(solve(), (885, 623));
}
