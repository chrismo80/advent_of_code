use std::{collections::*, path};

#[derive(Default)]
pub struct Graph<T>
{
    pub nodes: HashMap<T, HashMap<T, i32>>,
}

impl<T> Graph<T>
where
    T: Copy + Clone + PartialEq + Eq + std::hash::Hash,
{
    pub fn new() -> Self
    {
        Graph { nodes: HashMap::new() }
    }

    pub fn add_edge(&mut self, from: T, to: T, distance: i32)
    {
        self.nodes.entry(from).or_default().insert(to, distance);
        self.nodes.entry(to).or_default().insert(from, distance);
    }

    pub fn bfs(&self, start: T, end: T) -> Option<Vec<T>>
    {
        let mut visited = HashMap::new();
        let mut active = VecDeque::new();

        active.push_back(start);

        while let Some(mut current) = active.pop_front() {
            if current == end {
                active.clear();

                while current != start {
                    active.push_front(current);
                    current = visited[&current];
                }

                return Some(active.into());
            }

            for neighbor in self.neighbors(&current) {
                if let hash_map::Entry::Vacant(previous) = visited.entry(neighbor) {
                    previous.insert(current);
                    active.push_back(neighbor);
                }
            }
        }
        None
    }

    pub fn neighbors(&self, node: &T) -> Vec<T>
    {
        self.nodes[&node].keys().copied().collect()
    }

    pub fn all_paths(&self, current: T, end: T) -> Vec<Vec<T>>
    {
        let path = Vec::new();
        let condition = |path: Vec<T>, current: T| !path.contains(&current);

        self.all_paths_with_condition(current, end, condition, path)
    }

    pub fn all_paths_with_condition<C>(&self, current: T, end: T, condition: C, path: Vec<T>) -> Vec<Vec<T>>
    where
        C: Fn(Vec<T>, T) -> bool + Clone,
    {
        let mut paths = Vec::new();
        let mut path = path;

        if path.contains(&current) && !condition(path.clone(), current) {
            return paths;
        }

        path.push(current);

        if current == end {
            paths.push(path);
            return paths;
        }

        for node in self.neighbors(&current) {
            for sub_paths in self.all_paths_with_condition(node, end, condition.clone(), path.clone()) {
                paths.push(sub_paths);
            }
        }

        paths
    }
}
