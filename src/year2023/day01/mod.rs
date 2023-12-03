pub fn solve() -> (i64, i64)
{
    let input: Vec<&str> = include_str!("input.txt").lines().collect();

    let names = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let result1 = input.iter().map(|line| get_calibration_value(line)).sum::<i64>();
    let result2 = input.iter().map(|line| get_real_calibration_value(line, &names)).sum::<i64>();

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

fn get_real_calibration_value(text: &str, names: &[&str]) -> i64
{
    let mut replaced = text.to_owned();

    for (i, name) in names.iter().enumerate() {
        replaced = replaced.replace(name, format!("{}{}{}", name, i, name).as_str());
    }

    get_calibration_value(replaced.as_str())
}

#[test]
fn test()
{
    assert_eq!(solve(), (54634, 53855));
}
