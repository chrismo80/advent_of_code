use crate::extensions::converter::Matrix;
use crate::extensions::converter::Parser;

pub fn solve() -> (usize, usize)
{
    let mut input = include_str!("input.txt").to_char_grid();

    let result1 = get_north_load(&input, true);

    let mut loads = Vec::new();

    while get_cycle(&loads).is_none() {
        loads.push(cycle(&mut input));
    }

    let cycle = get_cycle(&loads).unwrap();

    let result2 = loads[(1_000_000_000 - cycle.0) % cycle.1 + cycle.0 - 1];

    println!("14\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

fn cycle(input: &mut Vec<Vec<char>>) -> usize
{
    tilt(input, "north");
    tilt(input, "west");
    tilt(input, "south");
    tilt(input, "east");

    get_north_load(input, false)
}

fn tilt(input: &mut Vec<Vec<char>>, direction: &str)
{
    let mut map = turn(input.to_vec(), direction, true);

    map = map.iter().map(|row| move_rocks(row)).collect();

    *input = turn(map, direction, false);
}

fn get_north_load(input: &[Vec<char>], shift_rocks: bool) -> usize
{
    let mut map = turn(input.to_vec(), "north", true);

    if shift_rocks {
        map = map.iter().map(|row| move_rocks(row)).collect();
    }

    let mut load = 0;

    for row in &map {
        for (i, &item) in row.iter().enumerate() {
            if item == 'O' {
                load += input.len() - i;
            }
        }
    }

    load
}

fn turn(map: Vec<Vec<char>>, direction: &str, pre: bool) -> Vec<Vec<char>>
{
    match (pre, direction) {
        (true, "west") | (false, "west") => map,
        (true, "south") | (false, "north") => map.rotate(),
        (true, "east") | (false, "east") => map.rotate().rotate(),
        (true, "north") | (false, "south") => map.rotate().rotate().rotate(),
        _ => panic!("Invalid direction"),
    }
}

fn move_rocks(row: &[char]) -> Vec<char>
{
    let mut line = row.iter().collect::<String>();

    while line.contains(".O") {
        line = line.replace(".O", "O.");
    }

    line.chars().collect()
}

fn get_cycle(set: &[usize]) -> Option<(usize, usize)>
{
    if set.len() < 5 {
        return None;
    }

    let rev: Vec<usize> = set.iter().rev().copied().collect();

    for i in 2..set.len() / 2 {
        if rev[..i] == rev[i..i * 2] {
            return Some((set.len() - i * 2, i));
        }
    }

    None
}

#[test]
fn test()
{
    assert_eq!(solve(), (106378, 90795));
}
