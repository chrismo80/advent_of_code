use std::collections::HashMap;

pub fn solve() -> (i64, i64)
{
    let input: Vec<&str> = include_str!("input.txt").lines().collect();

    let map = HashMap::from([
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
    ]);

    let result1 = input.iter().map(|line| get_calibration_value(line)).sum::<i64>();
    let result2 = input.iter().map(|line| get_real_calibration_value(line, &map)).sum::<i64>();

    println!("1\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

fn get_calibration_value(text: &str) -> i64
{
    let digits: Vec<char> = text.chars().filter(|c| c.is_ascii_digit()).collect();

    format!("{}{}", digits.first().unwrap(), digits.last().unwrap())
        .parse::<i64>()
        .unwrap()
}

fn get_real_calibration_value(text: &str, map: &HashMap<&str, &str>) -> i64
{
    let mut replaced = text.to_owned();

    for (key, value) in map.iter() {
        replaced = replaced.replace(key, value);
    }

    get_calibration_value(replaced.as_str())
}

#[test]
fn test()
{
    assert_eq!(solve(), (54634, 53855));
}
