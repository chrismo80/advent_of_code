use crate::extensions::converter::Converter;

pub fn solve() -> (String, i32)
{
    let mut discs = include_str!("input.txt").to_vec::<Disc>("\n");

    for d in 0..discs.len() {
        let parent_name = discs[d].name.clone();

        for c in 0..discs[d].children.len() {
            let child_name = discs[d].children[c].clone();

            let child_disc = discs.iter_mut().find(|d| d.name == child_name).unwrap();

            child_disc.set_parent(&parent_name);
        }
    }

    // find unbalanced discs
    let mut unbalanced = discs.iter().filter(|d| !d.is_balanced(&discs)).collect::<Vec<_>>();

    // get unbalanced disc with most parents
    unbalanced.sort_by_key(|d| d.count_parents(&discs));

    let unbalanced_disc_name = unbalanced.last().unwrap().name.clone();

    // get  children of found disc
    let mut unbalanced_children = discs
        .iter()
        .filter(|d| d.parent == Some(unbalanced_disc_name.clone()))
        .collect::<Vec<_>>();

    // sort children by total weight to find max / min
    unbalanced_children.sort_by_key(|d| d.total_weight(&discs));

    let diff =
        unbalanced_children.last().unwrap().total_weight(&discs) - unbalanced_children.first().unwrap().total_weight(&discs);

    let root = discs.iter().find(|d| d.parent.is_none()).unwrap().name.clone();

    let result1 = root;
    let result2 = unbalanced_children.last().unwrap().weight - diff;

    println!("7\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

#[test]
fn test()
{
    assert_eq!(solve(), ("cyrupz".to_string(), 193));
}

struct Disc
{
    name: String,
    weight: i32,
    children: Vec<String>,
    parent: Option<String>,
}

impl Disc
{
    fn new(name: &str, weight: i32, children: Vec<String>) -> Disc
    {
        Disc {
            name: name.to_string(),
            weight,
            children,
            parent: None,
        }
    }

    fn set_parent(&mut self, parent: &str)
    {
        self.parent = Some(parent.to_string());
    }

    fn count_parents(&self, discs: &[Disc]) -> usize
    {
        let mut count = 0;

        let mut parent = self.parent.clone();

        while let Some(p) = parent {
            count += 1;
            parent = discs.iter().find(|d| d.name == p).unwrap().parent.clone();
        }

        count
    }

    fn total_weight(&self, discs: &[Disc]) -> i32
    {
        let mut weight = self.weight;

        for child in &self.children {
            weight += discs.iter().find(|d| d.name == *child).unwrap().total_weight(discs);
        }

        weight
    }

    fn is_balanced(&self, discs: &[Disc]) -> bool
    {
        let mut weights = Vec::new();

        for child in &self.children {
            weights.push(discs.iter().find(|d| d.name == *child).unwrap().total_weight(discs));
        }

        weights.iter().all(|w| w == &weights[0])
    }
}

impl std::str::FromStr for Disc
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        let mut parts = s.split(" -> ");

        let name_weight = parts.next().unwrap().split(' ').collect::<Vec<_>>();

        let name = name_weight[0];
        let weight = name_weight[1].trim_matches(|c| c == '(' || c == ')').parse().unwrap();

        let children = if let Some(children) = parts.next() {
            children.split(", ").map(|c| c.to_string()).collect()
        }
        else {
            Vec::new()
        };

        Ok(Disc::new(name, weight, children))
    }
}
