use std::collections::*;

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

    pub fn dfs(&self, start: (usize, usize), end: (usize, usize)) -> Option<Vec<(usize, usize)>>
    {
        let mut queue = Vec::new();
        queue.push(start);

        let mut visited = HashMap::new();
        visited.insert(start, start);

        while let Some(current) = queue.pop() {
            if current == end {
                let mut path = Vec::new();
                let mut node = end;

                while node != start {
                    path.push(node);
                    node = visited[&node];
                }

                path.reverse();

                return Some(path);
            }

            for neighbor in self.neighbors(current) {
                if !visited.contains_key(&neighbor)
                    && (self.walkable)(&self.map[current.1][current.0], &self.map[neighbor.1][neighbor.0])
                {
                    visited.insert(neighbor, current);
                    queue.push(neighbor);
                }
            }
        }

        None
    }

    fn neighbors(&self, coord: (usize, usize)) -> Vec<(usize, usize)>
    {
        let mut neighbors = Vec::new();
        let (x, y) = coord;

        if y > 0 {
            neighbors.push((x, y - 1));
        }
        if x > 0 {
            neighbors.push((x - 1, y));
        }
        if y < self.map.len() - 1 {
            neighbors.push((x, y + 1));
        }
        if x < self.map[0].len() - 1 {
            neighbors.push((x + 1, y));
        }

        neighbors
    }
}
