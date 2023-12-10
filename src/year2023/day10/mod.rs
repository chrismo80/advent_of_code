use crate::extensions::converter::Parser;

pub fn solve() -> (usize, usize)
{
    let input = include_str!("input.txt").to_char_grid();

    let mut current = (91, 40); // set first pipe next to S manually
    let mut last = (91, 41); // check manually where S is
    let mut distance = 1;

    while input[current.1][current.0] != 'S' {
        let next = get_next_pipe(&input, current, last);

        last = current;
        current = next;
        distance += 1;
    }

    let result1 = distance / 2;
    let result2 = 1;

    println!("10\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

fn get_next_pipe(map: &[Vec<char>], current: (usize, usize), last: (usize, usize)) -> (usize, usize)
{
    let (x, y) = current;

    match map[y][x] {
        '|' if last.1 > current.1 => (x, y - 1),
        '|' if last.1 < current.1 => (x, y + 1),
        '-' if last.0 < current.0 => (x + 1, y),
        '-' if last.0 > current.0 => (x - 1, y),
        'F' if last.1 > current.1 => (x + 1, y),
        'F' if last.0 > current.0 => (x, y + 1),
        '7' if last.1 > current.1 => (x - 1, y),
        '7' if last.0 < current.0 => (x, y + 1),
        'J' if last.1 < current.1 => (x - 1, y),
        'J' if last.0 < current.0 => (x, y - 1),
        'L' if last.1 < current.1 => (x + 1, y),
        'L' if last.0 > current.0 => (x, y - 1),
        _ => panic!("Invalid pipe"),
    }
}

#[test]
fn test()
{
    assert_eq!(solve(), (0, 0));
}
