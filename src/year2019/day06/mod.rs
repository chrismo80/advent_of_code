use crate::path_finding::graph::Graph;
use rayon::prelude::*;

pub fn solve() -> (usize, usize)
{
    let input: Vec<Vec<&str>> = include_str!("input.txt").lines().map(|l| l.split(')').collect()).collect();

    let mut graph: Graph<&str> = Graph::new();

    input.iter().for_each(|planet| {
        graph.add_edge(planet[0], planet[1], 1);
    });

    let planets: Vec<&str> = graph.nodes.keys().copied().collect();

    let result1 = planets.par_iter().map(|p| graph.clone().bfs("COM", p).unwrap().len()).sum();
    let result2 = graph.bfs("YOU", "SAN").unwrap().len() - 2;

    println!("6\t{result1:<20}\t{result2:<20}");

    (result1, result2)
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
