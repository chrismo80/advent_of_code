use super::intcode_computer::*;
use crate::extensions::converter::Parser;
use std::collections::*;

pub fn solve() -> (usize, i64)
{
    let input = include_str!("input.txt").to_vec::<i64>(",");

    let result1 = part_1(&input);

    let (mut x, mut y, mut distance) = (0, 50, 0);
    let mut horizontal = true;

    while distance < 135 || horizontal {
        distance = 0;

        while get_value(&input, x, y) == 0 {
            match horizontal {
                true => x += 1,
                false => y += 1,
            }
        }
        while get_value(&input, x, y) == 1 {
            match horizontal {
                true => x += 1,
                false => y += 1,
            }
            distance += 1;
        }
        horizontal = !horizontal;
    }

    let result2 = part_2(&input, x, y);

    println!("19\t{result1:<20}\t{result2:<20}");

    //print(&beam);

    (result1, result2)
}

fn part_2(memory: &[i64], mut x: i64, mut y: i64) -> i64
{
    let (mut last, mut start_x) = (0, x);
    let ship = 100;

    let mut beam_histogram_x = HashMap::<i64, usize>::new();
    let mut beam_histogram_y = HashMap::<i64, usize>::new();

    let mut result2 = 0;

    while result2 == 0 {
        let value = get_value(memory, x, y);

        if value == 1 {
            *beam_histogram_x.entry(x).or_insert(0) += 1;
            *beam_histogram_y.entry(y).or_insert(0) += 1;
        }

        let beam_x = *beam_histogram_x.get(&x).unwrap_or(&0);
        let beam_y = *beam_histogram_y.get(&y).unwrap_or(&0);

        if beam_x >= ship && beam_y >= ship {
            result2 = ((x - ship as i64 + 1) * 10000) + (y - ship as i64 + 1);
        }

        if (value == 0 && last == 1) || (beam_x == 0 && x > 10) {
            y += 1;
            x = (start_x - 1).max(0);
        }

        if value == 1 && last == 0 {
            start_x = x;

            let known = *beam_histogram_y.get(&(y - 1)).unwrap_or(&0) / 2;
            *beam_histogram_y.entry(y).or_insert(0) += known;
            x += known as i64;
        }

        x += 1;
        last = value;
    }

    result2
}

fn part_1(memory: &[i64]) -> usize
{
    let (mut x, mut y) = (0, 0);
    let (mut last, mut start_x) = (0, 0);

    let mut beam_histogram_x = HashMap::<i64, usize>::new();
    let mut beam_histogram_y = HashMap::<i64, usize>::new();

    let mut result1 = 0;

    while x < 50 && y < 50 {
        let value = get_value(memory, x, y);

        if value == 1 {
            result1 += 1;

            *beam_histogram_x.entry(x).or_insert(0) += 1;
            *beam_histogram_y.entry(y).or_insert(0) += 1;
        }

        let beam_x = beam_histogram_x.get(&x).copied().unwrap_or(0);

        if (value == 0 && last == 1) || (beam_x == 0 && x > 10) {
            y += 1;
            x = (start_x - 1).max(0);
        }

        if value == 1 && last == 0 {
            start_x = x;
        }

        x += 1;
        last = value;
    }

    result1
}

fn get_value(memory: &[i64], x: i64, y: i64) -> i64
{
    let mut drone_system = IntCodeComputer::new(memory);

    drone_system.add_input(x);
    drone_system.add_input(y);

    drone_system.run();

    drone_system.get_output().unwrap()
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
