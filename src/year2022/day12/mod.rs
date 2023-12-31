use crate::extensions::converter::*;
use crate::path_finding::grid::*;
use rayon::prelude::*;

pub fn solve() -> (usize, usize)
{
    let mut map = include_str!("input.txt").to_char_grid();

    let mut start = (0, 0);
    let mut end = (0, 0);

    let mut part2_starts: Vec<(usize, usize)> = Vec::new();

    for row in 0..map.len() {
        for col in 0..map[0].len() {
            match map[row][col] {
                'a' => {
                    part2_starts.push((col, row));
                }
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

    let result1 = Grid::new(&map, walkable).bfs(start, end).unwrap().len();

    let result2 = part2_starts
        .par_iter()
        .map(|s| Grid::new(&map, Box::new(|c: &char, n: &char| *n as i32 - *c as i32 <= 1)).bfs(*s, end))
        .filter(|path| path.is_some())
        .map(|path| path.unwrap().len())
        .min()
        .unwrap();

    println!("12\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

#[test]
fn test()
{
    assert_eq!(solve(), (517, 512));
}
