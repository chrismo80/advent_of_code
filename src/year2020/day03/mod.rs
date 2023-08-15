pub fn solve() -> (usize, usize)
{
    let input: Vec<&str> = include_str!("input.txt").lines().collect();

    let results: Vec<usize> = (0..5)
        .map(|slope| (((slope * 2) + 1) % 8, (slope / 4) + 1))
        .map(|slope| count_trees(slope, &input))
        .collect();

    let result1 = results[1];
    let result2 = results.iter().product();

    println!("3\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

fn count_trees(slope: (usize, usize), forest: &Vec<&str>) -> usize
{
    let (mut x, mut y, mut c) = (0, 0, 0);

    while y < forest.len() {
        c += (forest[y].chars().nth(x).unwrap() == '#') as usize;
        y += slope.1;
        x += slope.0;
        x %= forest[0].len();
    }

    c
}

#[cfg(test)]
mod tests
{
    #[test]
    fn solve()
    {
        assert_eq!(super::solve(), (278, 9709761600));
    }
}
