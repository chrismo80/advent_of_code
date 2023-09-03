use std::collections::*;

#[derive(Default, Clone)]
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
        let mut previous = HashMap::new();
        let mut active = VecDeque::new();

        active.push_back(start);

        while let Some(mut current) = active.pop_front() {
            if current == end {
                active.clear();

                while current != start {
                    active.push_front(current);
                    current = previous[&current];
                }

                return Some(active.into());
            }

            for &neighbor in self.nodes[&current].keys() {
                if let hash_map::Entry::Vacant(entry) = previous.entry(neighbor) {
                    entry.insert(current);
                    active.push_back(neighbor);
                }
            }
        }
        None
    }
}
