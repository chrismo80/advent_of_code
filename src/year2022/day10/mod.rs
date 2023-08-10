pub fn solve() -> i32
{
    let input = include_str!("input.txt")
        .lines()
        .map(|line| {
            if line.starts_with("add") {
                (2, line.split(' ').collect::<Vec<&str>>()[1].parse::<i32>().unwrap())
            }
            else {
                ((line == "noop") as i32, 0)
            }
        })
        .flat_map(|command| {
            (0..command.0)
                .map(|cycle| command.1 * (cycle > 0) as i32)
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<i32>>();

    let mut x = vec![1];

    for change in input {
        x.push(x.last().unwrap() + change);
    }

    let result1 = (0..6).map(|c| signal_strength((c * 40) + 20, &x)).sum::<i32>();

    println!("10\t{result1}");

    for (i, x) in x.iter().enumerate() {
        print!(
            "{}{}",
            if i % 40 == 0 { "\r\n" } else { "" },
            if (i as i32 % 40 - x).abs() <= 1 { "#" } else { "." }
        );
    }

    result1
}

fn signal_strength(cycles: i32, x: &[i32]) -> i32
{
    x[(cycles - 1) as usize] * cycles
}

#[cfg(test)]
mod tests
{
    #[test]
    fn solve()
    {
        assert_eq!(super::solve(), 12740);
    }
}
