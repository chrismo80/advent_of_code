pub fn solve() -> (usize, usize)
{
    let input: Vec<Vec<usize>> = include_str!("input.txt")
        .lines()
        .map(|line| line.split_whitespace().map(|n| n.parse::<usize>().unwrap()).collect())
        .collect();

    let is_triangle = |sides: &[usize]| {
        sides[0] + sides[1] > sides[2] && sides[0] + sides[2] > sides[1] && sides[1] + sides[2] > sides[0]
    };

    let result1 = input.iter().filter(|sides| is_triangle(sides)).count();

    let result2 = input
        .iter()
        .map(|s| s[0])
        .chain(input.iter().map(|s| s[1]))
        .chain(input.iter().map(|s| s[2]))
        .collect::<Vec<usize>>()
        .chunks(3)
        .filter(|sides| is_triangle(sides))
        .count();

    println!("3\t{result1:<15}\t{result2:<15}");

    (result1, result2)
}

#[cfg(test)]
mod tests
{
    #[test]
    fn solve()
    {
        assert_eq!(super::solve(), (993, 1849));
    }
}
