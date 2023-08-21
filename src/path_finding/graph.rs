use std::collections::*;

#[derive(Default, Clone)]
pub struct Graph<T>
{
    pub nodes: HashMap<T, HashMap<T, i32>>,
    previous: HashMap<T, T>,
}

impl<T> Graph<T>
where
    T: Copy + Clone + PartialEq + Eq + std::hash::Hash,
{
    pub fn new() -> Self
    {
        Graph {
            nodes: HashMap::new(),
            previous: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, from: T, to: T, distance: i32)
    {
        self.nodes.entry(from).or_default().insert(to, distance);
        self.nodes.entry(to).or_default().insert(from, distance);
    }

    pub fn bfs(&mut self, start: T, end: T) -> Option<Vec<T>>
    {
        self.previous.clear();

        let mut active = VecDeque::new();
        active.push_back(start);

        while let Some(current) = active.pop_front() {
            if current == end {
                let mut path = Vec::new();
                let mut node = end;

                while node != start {
                    path.push(node);
                    node = self.previous[&node];
                }

                path.reverse();

                return Some(path);
            }

            for &neighbor in self.nodes[&current].keys() {
                if let hash_map::Entry::Vacant(entry) = self.previous.entry(neighbor) {
                    entry.insert(current);
                    active.push_back(neighbor);
                }
            }
        }

        None
    }
}
