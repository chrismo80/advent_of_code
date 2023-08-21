use crate::path_finding::graph::Graph;

pub fn solve() -> (i64, i64)
{
    let input: Vec<Vec<&str>> = include_str!("input.txt").lines().map(|l| l.split(')').collect()).collect();

    let mut graph: Graph<&str> = Graph::new();

    input.iter().for_each(|planet| {
        graph.add_edge(&planet[0], &planet[1], 1);
    });

    let left: Vec<&str> = input.iter().map(|l| l[0]).collect();
    let right: Vec<&str> = input.iter().map(|l| l[1]).collect();

    let com = right.iter().find(|&l| !left.contains(l)).unwrap().to_string();

    let result1 = 0;
    let result2 = 0;

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
