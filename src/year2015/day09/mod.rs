use iter_tools::Itertools;
use permute::*;
use std::collections::*;

pub fn solve() -> (usize, usize)
{
    let input: Vec<(&str, &str, usize)> = include_str!("input.txt")
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .map(|words| (words[0], words[2], words[4].parse::<usize>().unwrap()))
        .collect();

    let mut distances: HashMap<(&str, &str), usize> = HashMap::new();
    let mut locations: HashSet<&str> = HashSet::new();

    for (from, to, distance) in input.iter() {
        distances.insert((from, to), *distance);
        distances.insert((to, from), *distance);
        locations.insert(from);
        locations.insert(to);
    }

    let total_distance = |route: &[&str]| {
        (0..route.len() - 1)
            .map(|i| distances[&(route[i], route[i + 1])])
            .sum::<usize>()
    };

    let mut result1 = usize::MAX;
    let mut result2 = 0;

    for p in permutations_of(&locations.iter().collect_vec()) {
        let total = total_distance(&p.copied().copied().collect::<Vec<&str>>());
        result1 = result1.min(total);
        result2 = result2.max(total);
    }

    println!("9\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

#[cfg(test)]
mod tests
{
    #[test]
    fn solve()
    {
        assert_eq!(super::solve(), (141, 736));
    }
}
