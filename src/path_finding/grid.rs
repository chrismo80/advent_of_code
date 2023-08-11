use std::collections::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Coord(pub usize, pub usize);

pub struct Grid<T>
{
    map: Vec<Vec<T>>,
    walkable: Box<dyn Fn(&T, &T) -> bool>,
}

impl<T> Grid<T>
where
    T: std::fmt::Debug,
{
    pub fn new(map: Vec<Vec<T>>, walkable: Box<dyn Fn(&T, &T) -> bool>) -> Self
    {
        Grid { map, walkable }
    }

    pub fn bfs(&self, start: Coord, end: Coord) -> Option<Vec<Coord>>
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
        let (x, y) = (coord.0, coord.1);

        if y > 0 {
            neighbors.push(Coord(x, y - 1));
        }
        if x > 0 {
            neighbors.push(Coord(x - 1, y));
        }
        if y < self.map.len() - 1 {
            neighbors.push(Coord(x, y + 1));
        }
        if x < self.map[0].len() - 1 {
            neighbors.push(Coord(x + 1, y));
        }

        neighbors
    }
}
