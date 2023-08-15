pub fn solve() -> (usize, usize)
{
    let input: Vec<usize> = include_str!("input.txt").lines().map(|l| l.parse().unwrap()).collect();

    let mut result1 = 0;
    let mut result2 = 0;

    for x in &input {
        if result1 > 0 && result2 > 0 {
            break;
        }

        for y in &input {
            if result1 == 0 && x + y == 2020 {
                result1 = x * y;
            }

            if result2 > 0 {
                continue;
            }

            for z in &input {
                if result2 == 0 && x + y + z == 2020 {
                    result2 = x * y * z;
                    break;
                }
            }
        }
    }

    println!("1\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

#[cfg(test)]
mod tests
{
    #[test]
    fn solve()
    {
        assert_eq!(super::solve(), (437931, 157667328));
    }
}
