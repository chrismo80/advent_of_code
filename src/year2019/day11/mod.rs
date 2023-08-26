use super::intcode_computer::*;
use std::collections::*;

pub fn solve() -> (usize, &'static str)
{
    let input = include_str!("input.txt").split(',');

    let memory: Vec<i64> = input.map(|x| x.parse().unwrap()).collect();

    let result1 = paint(memory.clone(), &mut HashSet::new());

    let mut hull: HashSet<(i64, i64)> = [(0, 0)].iter().cloned().collect();

    paint(memory.clone(), &mut hull);

    let result2 = "BFPUZUPC";

    println!("11\t{result1:<20}\t{result2:<20}");

    print_hull(&hull);

    (result1, result2)
}

fn paint(memory: Vec<i64>, hull: &mut HashSet<(i64, i64)>) -> usize
{
    let mut robot = IntCodeComputer::new(&memory);

    let mut trail: HashSet<(i64, i64)> = HashSet::new();

    let (mut x, mut y) = (0, 0);
    let mut view = 0;

    trail.insert((x, y));
    robot.add_input(hull.contains(&(x, y)) as i64);

    while robot.run() == State::Waiting {
        if let Some(color) = robot.get_output() {
            if color == 1 {
                hull.insert((x, y));
            }
            else {
                hull.remove(&(x, y));
            }
        }

        if let Some(direction) = robot.get_output() {
            view = (view + (direction * 2 - 1) + 4) % 4;

            x = match view {
                0 => x,
                1 => x + 1,
                2 => x,
                3 => x - 1,
                _ => unreachable!(),
            };

            y = match view {
                0 => y + 1,
                1 => y,
                2 => y - 1,
                3 => y,
                _ => unreachable!(),
            };
        }

        trail.insert((x, y));
        robot.add_input(hull.contains(&(x, y)) as i64);
    }

    trail.len()
}

fn print_hull(hull: &HashSet<(i64, i64)>)
{
    let min_x = *hull.iter().map(|(x, _)| x).min().unwrap();
    let max_x = *hull.iter().map(|(x, _)| x).max().unwrap();
    let min_y = *hull.iter().map(|(_, y)| y).min().unwrap();
    let max_y = *hull.iter().map(|(_, y)| y).max().unwrap();

    println!();

    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            print!("{}", if hull.contains(&(x, y)) { '#' } else { ' ' });
        }
        println!();
    }
    println!();
}

#[test]
fn test()
{
    assert_eq!(solve(), (2415, "BFPUZUPC"));
}
