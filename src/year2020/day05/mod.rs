pub fn solve() -> (i32, i32)
{
    let input = include_str!("input.txt").lines();

    let mut seat_ids: Vec<i32> = input
        .map(|code| 8 * position(&code[..7], 'B') + position(&code[7..], 'R'))
        .collect();

    seat_ids.sort();

    let result1 = *seat_ids.iter().max().unwrap();
    let result2 = seat_ids.as_slice().windows(2).find(|&seats| seats[1] - seats[0] > 1).unwrap()[0] + 1;

    println!("5\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

fn position(code: &str, direction: char) -> i32
{
    let mut current = 2_i32.pow(code.len() as u32);
    let mut size = current / 2;

    for c in code.chars() {
        if c != direction {
            current -= size;
        }
        size /= 2;
    }

    current - 1
}

#[test]
fn test()
{
    assert_eq!(solve(), (885, 623));
}
