use crate::extensions::converter::Parser;
use std::collections::{HashSet, VecDeque};

pub fn solve() -> (u32, usize)
{
    let input = include_str!("input.txt").to_int_grid();

    let neighbors = |x: usize, y: usize| -> Vec<(usize, usize)> {
        let mut result = Vec::new();

        if y > 0 {
            result.push((x, y - 1));
        }
        if x > 0 {
            result.push((x - 1, y));
        }
        if y < input.len() - 1 {
            result.push((x, y + 1));
        }
        if x < input[0].len() - 1 {
            result.push((x + 1, y));
        }

        result
    };

    let mut sinks = Vec::new();

    for x in 0..input[0].len() {
        for y in 0..input.len() {
            if neighbors(x, y).iter().map(|n| input[n.1][n.0]).all(|n| n > input[y][x]) {
                sinks.push((x, y));
            }
        }
    }

    // BFS
    let baisin_size = |x: usize, y: usize| -> usize {
        let mut visited = HashSet::new();
        let mut active = VecDeque::new();

        active.push_back((x, y));

        while let Some((x, y)) = active.pop_front() {
            for neighbor in neighbors(x, y)
                .iter()
                .filter(|n| input[n.1][n.0] < 9 && visited.insert((n.0, n.1)))
            {
                active.push_back(*neighbor)
            }
        }

        visited.len()
    };

    let mut baisins: Vec<usize> = sinks.iter().map(|s| baisin_size(s.0, s.1)).collect();

    baisins.sort();

    let result1 = sinks.iter().map(|s| input[s.1][s.0]).sum::<u32>() + sinks.len() as u32;
    let result2 = baisins.iter().rev().take(3).product();

    println!("9\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

#[test]
fn test()
{
    assert_eq!(solve(), (486, 1059300));
}
