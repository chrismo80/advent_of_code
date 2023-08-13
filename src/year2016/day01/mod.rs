use std::collections::*;

pub fn solve() -> (usize, usize)
{
    let input: Vec<&str> = include_str!("input.txt").split(", ").collect();

    let (mut x, mut y) = (0, 0);
    let mut trail: HashSet<(i32, i32)> = HashSet::new();
    let mut view = 0;
    let mut result2 = (0, 0);

    for step in input {
        view = match step.chars().next().unwrap() {
            'L' => (view + 4 - 1) % 4,
            'R' => (view + 4 + 1) % 4,
            _ => view,
        };

        for _ in 0..step[1..].parse::<i32>().unwrap() {
            x = match view {
                1 => x + 1,
                3 => x - 1,
                _ => x,
            };

            y = match view {
                0 => y - 1,
                2 => y + 1,
                _ => y,
            };

            if !trail.insert((x, y)) {
                result2 = match result2 {
                    (0, 0) => (x, y),
                    _ => result2,
                };
            }
        }
    }

    let result1 = (x.abs() + y.abs()) as usize;
    let result2 = (result2.0.abs() + result2.1.abs()) as usize;

    println!("1\t{result1}\t\t{result2}");

    (result1, result2)
}

#[cfg(test)]
mod tests
{
    #[test]
    fn solve()
    {
        assert_eq!(super::solve(), (278, 161));
    }
}
