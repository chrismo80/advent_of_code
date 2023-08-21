use std::collections::*;

pub fn main()
{
    let mut g: Graph<&str> = Graph::new();

    g.add_edge(&"0", &"1", 1);
    g.add_edge(&"0", &"2", 1);
    g.add_edge(&"1", &"2", 1);
    g.add_edge(&"1", &"3", 1);
    g.add_edge(&"4", &"3", 1);
    g.add_edge(&"4", &"2", 1);

    if let Some(path) = g.bfs(&"0", &"3") {
        println!("{:?}", path);
    }
    else {
        println!("No path found");
    }
}

#[derive(Default)]
pub struct Graph<T>
{
    nodes: HashMap<T, HashMap<T, i32>>,
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

    pub fn add_edge(&mut self, from: &T, to: &T, distance: i32)
    {
        self.nodes.entry(*from).or_default().insert(*to, distance);
        self.nodes.entry(*to).or_default().insert(*from, distance);
    }

    pub fn bfs(&mut self, start: &T, end: &T) -> Option<Vec<T>>
    {
        self.previous.clear();

        let mut active = VecDeque::new();
        active.push_back(*start);

        while let Some(current) = active.pop_front() {
            if current == *end {
                let mut path = Vec::new();
                let mut node = *end;

                while node != *start {
                    path.push(node);
                    node = self.previous[&node];
                }

                path.reverse();

                return Some(path);
            }

            let neighbors: Vec<T> = self.nodes[&current]
                .keys()
                .copied()
                .filter(|node| !self.previous.contains_key(node))
                .collect();

            for neighbor in neighbors {
                self.previous.insert(neighbor, current);
                active.push_back(neighbor);
            }
        }

        None
    }
}
