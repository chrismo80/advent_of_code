pub fn solve() -> (usize, usize)
{
    let mut map = include_str!("input.txt")
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut start = Coord(0, 0);
    let mut end = Coord(0, 0);

    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col] == 'S' {
                start = Coord(col, row);
                map[row][col] = 'a';
            }

            if map[row][col] == 'E' {
                end = Coord(col, row);
                map[row][col] = 'z';
            }
        }
    }

    let walkable = Box::new(|current: &char, neighbor: &char| *neighbor as i32 - *current as i32 <= 1);
    let grid = Grid::new(map, walkable);

    let result1 = grid.bfs(start, end).unwrap().len() - 1;
    let result2 = 0;

    println!("12\t{result1}\t{result2}");

    (result1, result2)
}

#[cfg(test)]
mod tests
{
    #[test]
    fn solve()
    {
        assert_eq!(super::solve(), (0, 0));
    }
}

use std::collections::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Coord(usize, usize);

struct Grid<T>
{
    map: Vec<Vec<T>>,
    walkable: Box<dyn Fn(&T, &T) -> bool>,
}

impl<T> Grid<T>
where
    T: std::fmt::Debug,
{
    fn new(map: Vec<Vec<T>>, walkable: Box<dyn Fn(&T, &T) -> bool>) -> Self
    {
        Grid { map, walkable }
    }

    fn bfs(&self, start: Coord, end: Coord) -> Option<Vec<Coord>>
    {
        let mut queue = VecDeque::new();
        queue.push_back(vec![start]);
        let mut visited = HashMap::new();
        visited.insert(start, start);

        while let Some(path) = queue.pop_front() {
            let current = path.last().unwrap();

            if *current == end {
                return Some(path);
            }

            for neighbor in self.neighbors(*current) {
                if !visited.contains_key(&neighbor)
                    && (self.walkable)(&self.map[current.1][current.0], &self.map[neighbor.1][neighbor.0])
                {
                    visited.insert(neighbor, *current);
                    let mut new_path = path.clone();
                    new_path.push(neighbor);
                    queue.push_back(new_path);
                }
            }
        }

        None
    }

    fn neighbors(&self, coord: Coord) -> Vec<Coord>
    {
        let mut neighbors = Vec::new();
        let (x, y) = (coord.0 as isize, coord.1 as isize);

        if y > 0 {
            neighbors.push(Coord(x as usize, (y - 1) as usize));
        }
        if x > 0 {
            neighbors.push(Coord((x - 1) as usize, y as usize));
        }
        if y < self.map.len() as isize - 1 {
            neighbors.push(Coord(x as usize, (y + 1) as usize));
        }
        if x < self.map[0].len() as isize - 1 {
            neighbors.push(Coord((x + 1) as usize, y as usize));
        }

        neighbors
    }
}
