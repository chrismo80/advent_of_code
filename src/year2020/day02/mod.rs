pub fn solve() -> (usize, usize)
{
    // C#:

    // var input = File.ReadAllLines("AdventOfCode/2020/02/Input.txt").Select(line =>
    //     (
    //         Left: int.Parse(line.Split(':')[0].Split(' ')[0].Split('-')[0]),
    //         Right: int.Parse(line.Split(':')[0].Split(' ')[0].Split('-')[1]),
    //         Char: line.Split(':')[0][^1..][0],
    //         Password: line.Split(": ")[1]
    //     )).ToList();

    // var result1 = input
    //     .Select(x => (Specs: x, Count: x.Password.Count(c => c == x.Char)))
    //     .Count(c => c.Count >= c.Specs.Left && c.Count <= c.Specs.Right);

    // var result2 = input
    //     .Count(p => p.Password[p.Left - 1] == p.Char ^ p.Password[p.Right - 1] == p.Char);

    // Console.WriteLine($"Part 1: {result1} (418), Part 2: {result2} (616)");

    // Rust:
    let input: Vec<&str> = include_str!("input.txt").lines().collect();

    let result1 = input
        .iter()
        .map(|line| {
            let parts = line.split(": ").collect::<Vec<_>>();
            let specs = parts[0].split(' ').collect::<Vec<_>>();
            let range = specs[0].split('-').collect::<Vec<_>>();
            let left = range[0].parse::<usize>().unwrap();
            let right = range[1].parse::<usize>().unwrap();
            let char = specs[1].chars().next().unwrap();
            let password = parts[1];
            (left, right, char, password)
        })
        .filter(|(left, right, char, password)| {
            let count = password.chars().filter(|c| c == char).count();
            count >= *left && count <= *right
        })
        .count();

    let result2 = input
        .iter()
        .map(|line| {
            let parts = line.split(": ").collect::<Vec<_>>();
            let specs = parts[0].split(' ').collect::<Vec<_>>();
            let range = specs[0].split('-').collect::<Vec<_>>();
            let left = range[0].parse::<usize>().unwrap();
            let right = range[1].parse::<usize>().unwrap();
            let char = specs[1].chars().next().unwrap();
            let password = parts[1];
            (left, right, char, password)
        })
        .filter(|(left, right, char, password)| {
            (password.chars().nth(*left - 1).unwrap() == *char) ^ (password.chars().nth(*right - 1).unwrap() == *char)
        })
        .count();

    println!("2\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

#[cfg(test)]
mod tests
{
    #[test]
    fn solve()
    {
        assert_eq!(super::solve(), (418, 616));
    }
}
