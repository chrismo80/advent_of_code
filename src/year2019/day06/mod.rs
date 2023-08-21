use crate::path_finding::graph::Graph;
use std::collections::HashMap;

pub fn solve() -> (usize, usize)
{
    let input: Vec<Vec<&str>> = include_str!("input.txt").lines().map(|l| l.split(')').collect()).collect();

    let mut graph: Graph<&str> = Graph::new();

    input.iter().for_each(|planet| {
        graph.add_edge(planet[0], planet[1], 1);
    });

    let planets: Vec<&str> = graph.nodes.keys().copied().collect();

    let result1 = planets.iter().map(|planet| graph.bfs("COM", planet).unwrap().len()).sum();
    let result2 = graph.bfs("YOU", "SAN").unwrap().len() - 2;

    println!("6\t{result1:<20}\t{result2:<20}");

    //println!("Alternative: {:?}", alternative(input));

    (result1, result2)
}

fn alternative(input: Vec<Vec<&str>>) -> usize
{
    let mut hash_map = HashMap::new();

    for line in input.iter() {
        hash_map.insert(line[1], line[0]);
    }

    hash_map.keys().map(|key| count(&hash_map, key)).sum()
}

fn count(map: &HashMap<&str, &str>, item: &str) -> usize
{
    map.get(item).map(|item| count(map, item) + 1).unwrap_or(0)
}

#[cfg(test)]
mod tests
{
    #[test]
    fn solve()
    {
        assert_eq!(super::solve(), (117672, 277));
    }
}
