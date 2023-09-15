use std::collections::HashSet;

pub fn solve() -> (usize, usize)
{
    let input: Vec<char> = include_str!("input.txt").chars().collect();

    let mut houses1 = vec![(0, 0)];
    let mut houses2 = vec![(0, 0)];
    let mut current = (0, 0);

    for direction in input.iter() {
        next(&mut current, direction);
        houses1.push(current);
    }

    current = (0, 0);
    for direction in input.iter().step_by(2) {
        next(&mut current, direction);
        houses2.push(current);
    }

    current = (0, 0);
    for direction in input.iter().skip(1).step_by(2) {
        next(&mut current, direction);
        houses2.push(current);
    }

    let result1 = houses1.iter().collect::<HashSet<_>>().len();
    let result2 = houses2.iter().collect::<HashSet<_>>().len();

    println!("3\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

fn next(current: &mut (i32, i32), direction: &char)
{
    match direction {
        'v' => current.1 += 1,
        '^' => current.1 -= 1,
        '>' => current.0 += 1,
        '<' => current.0 -= 1,
        _ => panic!("Invalid direction"),
    };
}

#[test]
fn test()
{
    assert_eq!(solve(), (2592, 2360));
}
