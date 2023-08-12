use crate::path_finding::grid::*;
use rayon::prelude::*;

pub fn solve() -> (usize, usize)
{
    let mut map = include_str!("input.txt")
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut start = (0, 0);
    let mut end = (0, 0);

    let mut part2_starts: Vec<((usize, usize), Vec<Vec<char>>)> = Vec::new();

    for row in 0..map.len() {
        for col in 0..map[0].len() {
            match map[row][col] {
                'a' => part2_starts.push(((col, row), map.clone())),
                'S' => {
                    start = (col, row);
                    map[row][col] = 'a';
                }
                'E' => {
                    end = (col, row);
                    map[row][col] = 'z';
                }
                _ => {}
            }
        }
    }

    let walkable = Box::new(|current: &char, neighbor: &char| *neighbor as i32 - *current as i32 <= 1);
    let grid = Grid::new(map, walkable);

    let result1 = grid.bfs(start, end).unwrap().len();
    let result2 = part2_starts
        .par_iter()
        .map(|s| Grid::new(s.1.clone(), Box::new(|c: &char, n: &char| *n as i32 - *c as i32 <= 1)).bfs(s.0, end))
        .filter(|path| path.is_some())
        .map(|path| path.unwrap().len())
        .min()
        .unwrap();

    println!("12\t{result1}\t\t{result2}");

    (result1, result2)
}

#[cfg(test)]
mod tests
{
    #[test]
    fn solve()
    {
        assert_eq!(super::solve(), (517, 512));
    }
}
