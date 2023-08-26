use super::intcode_computer::*;
use std::collections::*;

pub fn solve() -> (usize, i64)
{
    let input = include_str!("input.txt").split(',');

    let memory: Vec<i64> = input.map(|x| x.parse().unwrap()).collect();

    let mut beam = HashSet::<(i64, i64)>::new();

    let (mut x, mut y, mut last, mut start_x, ship) = (0, 0, 0, 0, 100);

    let mut beam_histogram_x = HashMap::<i64, usize>::new();
    let mut beam_histogram_y = HashMap::<i64, usize>::new();

    let mut result2 = 0;

    while result2 == 0 {
        let mut drone_system = IntCodeComputer::new(&memory);

        drone_system.add_input(x);
        drone_system.add_input(y);

        drone_system.run();

        let value = drone_system.get_output().unwrap();

        if value == 1 {
            beam.insert((x, y));
            *beam_histogram_x.entry(x).or_insert(0) += 1;
            *beam_histogram_y.entry(y).or_insert(0) += 1;
        }

        let beam_x = beam_histogram_x.get(&x).copied().unwrap_or(0);
        let beam_y = beam_histogram_y.get(&y).copied().unwrap_or(0);

        if beam_x >= ship && beam_y >= ship {
            result2 = ((x - ship as i64 + 1) * 10000) + (y - ship as i64 + 1);
        }

        if (value == 0 && last == 1) || (beam_x == 0 && x > 10) {
            y += 1;
            x = (start_x - 5).max(0);
        }

        if value == 1 && last == 0 {
            start_x = x;
        }

        x += 1;
        last = value;
    }

    let result1 = beam.len();

    //print(&beam);

    (result1, result2)
}

fn print(beam: &HashSet<(i64, i64)>)
{
    let beam_x_min = beam.iter().map(|b| b.0).min().unwrap();
    let beam_x_max = beam.iter().map(|b| b.0).max().unwrap();
    let beam_y_min = beam.iter().map(|b| b.1).min().unwrap();
    let beam_y_max = beam.iter().map(|b| b.1).max().unwrap();

    for y in beam_y_min..=beam_y_max {
        for x in beam_x_min..=beam_x_max {
            if beam.contains(&(x, y)) {
                print!("#");
            }
            else {
                print!(".");
            }
        }

        println!();
    }
}

#[test]
fn test()
{
    assert_eq!(solve(), (126, 11351625));
}
