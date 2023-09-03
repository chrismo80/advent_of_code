use crate::extensions::input_parser::*;

pub fn solve() -> (i64, i64)
{
    let input = include_str!("input.txt").to_vec::<i64>();

    let result1 = input.iter().map(|mass| mass / 3 - 2).sum();
    let result2 = input.iter().map(|mass| get_total_fuel(*mass)).sum();

    println!("1\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

fn get_total_fuel(mass: i64) -> i64
{
    let mut total_fuel = 0;
    let mut fuel = mass / 3 - 2;

    while fuel > 0 {
        total_fuel += fuel;
        fuel = fuel / 3 - 2;
    }

    total_fuel
}

#[test]
fn test()
{
    assert_eq!(solve(), (3266516, 4896902));
}
