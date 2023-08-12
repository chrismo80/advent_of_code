pub fn solve() -> String
{
    let lines = include_str!("input.txt").lines();

    let number = lines.map(|snafu| convert_from(snafu.to_string(), "=-012", -2)).sum();

    let result1 = convert_to(number, "=-012", -2).to_string();

    println!("25\t{result1}");

    result1
}

fn convert_from(text: String, base: &str, offset: i64) -> i64
{
    let (mut value, mut power) = (0, 1);

    for c in text.chars().rev() {
        value += power * (base.find(c).unwrap() as i64 + offset);
        power *= base.len() as i64;
    }

    value
}

fn convert_to(value: i64, base: &str, offset: i64) -> String
{
    let mut text = String::new();
    let mut value = value;

    while value > 0 {
        value -= offset;
        text = base.chars().nth(value as usize % base.len()).unwrap().to_string() + &text;
        value /= base.len() as i64;
    }

    text
}

#[cfg(test)]
mod tests
{
    #[test]
    fn solve()
    {
        assert_eq!(super::solve(), "2=0-2-1-0=20-01-2-20");
    }
}
