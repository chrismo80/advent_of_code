use crate::extensions::converter::Parser;
use crate::path_finding::graph::Graph;
use iter_tools::Itertools;

pub fn solve() -> (usize, usize)
{
    let input = include_str!("input.txt").to_vec_of_vec::<String>("\n", "-");

    let mut graph = Graph::new();

    input.iter().for_each(|l| graph.add_edge(l[0].as_str(), l[1].as_str(), 1));

    let condition1 = |_: &Vec<&str>, current: &str| current.chars().next().unwrap().is_uppercase();
    let condition2 = |path: &Vec<&str>, current: &str| {
        current.chars().next().unwrap().is_uppercase()
            || path.iter().filter(|&e| e.chars().next().unwrap().is_lowercase()).all_unique() && current != "start"
    };

    let result1 = graph.all_paths_with_condition("start", "end", condition1, Vec::new()).len();
    let result2 = graph.all_paths_with_condition("start", "end", condition2, Vec::new()).len();

    println!("11\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

#[test]
fn test()
{
    assert_eq!(solve(), (3510, 122880));
}
