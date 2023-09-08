use crate::extensions::{converter::Converter, permutations::*};
use std::collections::*;

pub fn solve() -> (usize, usize)
{
    let map = include_str!("input.txt").to_char_grid();

    let grid = crate::path_finding::grid::Grid::new(&map, Box::new(|_, next| *next != '#'));

    // get node positions from map (for BFS start/end)
    let nodes: HashMap<usize, (usize, usize)> = (0..map.len())
        .flat_map(|y| (0..map[0].len()).map(move |x| (x, y)))
        .filter(|(x, y)| map[*y][*x].is_ascii_digit())
        .map(|(x, y)| (map[y][x].to_digit(10).unwrap() as usize, (x, y)))
        .collect();

    let mut distances: HashMap<(usize, usize), usize> = HashMap::new();

    // calculate the distances for all possible edges
    for (from, to) in nodes
        .keys()
        .flat_map(|f| nodes.keys().map(move |t| (f, t)))
        .filter(|(f, t)| f < t)
    {
        let distance = grid.bfs(nodes[from], nodes[to]).unwrap().len();

        // store for both directions
        distances.insert((*from, *to), distance);
        distances.insert((*to, *from), distance);
    }

    // get nodes as list except 0 (start/end)
    let locations: Vec<usize> = nodes.keys().filter(|n| **n != 0).copied().collect();

    let mut routes1: Vec<usize> = Vec::new();
    let mut routes2: Vec<usize> = Vec::new();

    // closure for calculating total distance for a route
    let total_distance = |route: Vec<usize>| -> usize { route.windows(2).map(|w| distances[&(w[0], w[1])]).sum() };

    // get all node permutations except node 0 (append/prepend it later as start/end points)
    for permutation in locations.permutations().iter() {
        let mut route: VecDeque<usize> = permutation.iter().copied().collect();

        route.push_back(0);
        routes1.push(total_distance(route.clone().into_iter().collect()));

        route.push_front(0);
        routes2.push(total_distance(route.clone().into_iter().collect()));
    }

    let result1 = *routes1.iter().min().unwrap();
    let result2 = *routes2.iter().min().unwrap();

    println!("24\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

#[test]
fn test()
{
    assert_eq!(solve(), (498, 804));
}
