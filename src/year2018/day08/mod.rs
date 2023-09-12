use crate::extensions::converter::Converter;

pub fn solve() -> (usize, usize)
{
    let mut input = include_str!("input.txt").to_vec::<usize>(" ");

    let root = Node::new(&mut input);

    let result1 = root.metadata_sum();
    let result2 = root.value();

    println!("8\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

struct Node
{
    children: Vec<Node>,
    metadata: Vec<usize>,
}

impl Node
{
    fn new(input: &mut Vec<usize>) -> Node
    {
        let mut children = Vec::new();
        let mut metadata = Vec::new();

        let c = input.remove(0);
        let m = input.remove(0);

        (0..c).for_each(|_| children.push(Node::new(input)));
        (0..m).for_each(|_| metadata.push(input.remove(0)));

        Node { metadata, children }
    }

    fn count(&self) -> usize
    {
        self.children.iter().map(|c| c.count()).sum::<usize>() + self.children.len()
    }

    fn metadata_sum(&self) -> usize
    {
        self.children.iter().map(|c| c.metadata_sum()).sum::<usize>() + self.metadata.iter().sum::<usize>()
    }

    fn value(&self) -> usize
    {
        if self.children.is_empty() {
            return self.metadata.iter().sum();
        }

        self.metadata
            .iter()
            .filter(|&m| m - 1 < self.children.len())
            .map(|&m| self.children[m - 1].value())
            .sum()
    }
}

#[test]
fn test()
{
    assert_eq!(solve(), (41028, 20849));
}
