use super::intcode_computer::*;
use std::collections::*;

pub fn solve() -> (i32, i64)
{
    let input = include_str!("input.txt").split(',');

    let mut memory: Vec<i64> = input.map(|x| x.parse().unwrap()).collect();
    memory[0] = 2;

    let mut robot = IntCodeComputer::new(&memory);

    let commands = [
        "A,B,A,C,A,B,C,B,C,B\n",
        "R,8,L,10,L,12,R,4\n",
        "R,8,L,12,R,4,R,4\n",
        "R,8,L,10,R,8\n",
        "n\n",
    ];

    for command in commands.iter() {
        for c in command.chars() {
            robot.add_input(c as i64);
        }
    }

    robot.run();

    let mut scaffold: HashSet<(i32, i32)> = HashSet::new();
    let (mut x, mut y) = (0, 0);

    let result2 = *robot.outputs.iter().last().unwrap();

    while let Some(value) = robot.get_output() {
        match value {
            10 => {
                x = -1;
                y += 1;
            }
            35 => {
                if y < 50 {
                    scaffold.insert((x, y));
                }
            }
            _ => {}
        }

        x += 1;
    }

    let result1 = scaffold
        .iter()
        .filter(|(x, y)| is_intersection(&scaffold, *x, *y))
        .map(|(x, y)| x * y)
        .sum::<i32>();

    println!("17\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

fn is_intersection(scaffold: &HashSet<(i32, i32)>, x: i32, y: i32) -> bool
{
    scaffold.contains(&(x + 1, y))
        && scaffold.contains(&(x - 1, y))
        && scaffold.contains(&(x, y + 1))
        && scaffold.contains(&(x, y - 1))
}

#[test]
fn test()
{
    assert_eq!(solve(), (4044, 893283));
}
