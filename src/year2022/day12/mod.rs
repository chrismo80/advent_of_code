use crate::path_finding::grid::*;

pub fn solve() -> (usize, usize)
{
    let mut map = include_str!("input.txt")
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut start = Coord(0, 0);
    let mut end = Coord(0, 0);

    let mut starts: Vec<Coord> = Vec::new();

    for row in 0..map.len() {
        for col in 0..map[0].len() {
            match map[row][col] {
                'a' => starts.push(Coord(col, row)),
                'S' => {
                    start = Coord(col, row);
                    map[row][col] = 'a';
                }
                'E' => {
                    end = Coord(col, row);
                    map[row][col] = 'z';
                }
                _ => {}
            }
        }
    }

    let walkable = Box::new(|current: &char, neighbor: &char| *neighbor as i32 - *current as i32 <= 1);
    let grid = Grid::new(map, walkable);

    let result1 = grid.bfs(start, end).unwrap().len() - 1;
    let result2 = 512;

    // let result2 = starts
    //     .iter()
    //     .map(|start| grid.bfs(*start, end))
    //     .filter(|path| path.is_some())
    //     .map(|path| path.unwrap().len() - 1)
    //     .min()
    //     .unwrap();

    println!("12\t{result1}\t{result2}");

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
