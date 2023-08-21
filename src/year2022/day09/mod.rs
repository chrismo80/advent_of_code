use std::collections::*;

pub fn solve() -> (usize, usize)
{
    let input = include_str!("input.txt")
        .lines()
        .map(|l| {
            (
                l.split_whitespace().collect::<Vec<&str>>()[0].chars().next().unwrap(),
                l.split_whitespace().collect::<Vec<&str>>()[1].parse::<i32>().unwrap(),
            )
        })
        .collect::<Vec<(char, i32)>>();

    let result1 = run(&input, &mut [(0, 0); 2]);
    let result2 = run(&input, &mut [(0, 0); 10]);

    println!("9\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

fn run(moves: &[(char, i32)], rope: &mut [(i32, i32)]) -> usize
{
    moves
        .iter()
        .flat_map(|m| (0..m.1).map(|_| move_rope(rope, m.0)).collect::<Vec<(i32, i32)>>())
        .collect::<HashSet<(i32, i32)>>()
        .len()
}

fn move_rope(rope: &mut [(i32, i32)], direction: char) -> (i32, i32)
{
    rope[0] = move_head(rope[0], direction);

    for i in 1..rope.len() {
        rope[i] = move_tail(rope[i - 1], rope[i]);
    }

    *rope.last().unwrap()
}

fn move_head(pos: (i32, i32), direction: char) -> (i32, i32)
{
    match direction {
        'U' => (pos.0 - 1, pos.1),
        'D' => (pos.0 + 1, pos.1),
        'L' => (pos.0, pos.1 - 1),
        'R' => (pos.0, pos.1 + 1),
        _ => panic!("Invalid direction"),
    }
}

fn move_tail(prev: (i32, i32), next: (i32, i32)) -> (i32, i32)
{
    match done(prev, next) {
        true => next,
        false => (next.0 + (prev.0 - next.0).signum(), next.1 + (prev.1 - next.1).signum()),
    }
}

fn done(prev: (i32, i32), next: (i32, i32)) -> bool
{
    (prev.0 - next.0).abs() < 2 && (prev.1 - next.1).abs() < 2
}

#[test]
fn test()
{
    assert_eq!(solve(), (6067, 2471));
}
