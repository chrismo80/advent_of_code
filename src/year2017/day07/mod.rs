use crate::extensions::converter::Converter;

pub fn solve() -> (String, i32)
{
    let mut discs = include_str!("input.txt").to_vec::<Disc>("\n");

    for d in 0..discs.len() {
        let parent_name = discs[d].name.clone();

        for c in 0..discs[d].children.len() {
            let child_name = discs[d].children[c].clone();

            discs
                .iter_mut()
                .find(|disc| disc.name == child_name)
                .unwrap()
                .set_parent(&parent_name);
        }
    }

    // set total weight for each disc
    for d in 0..discs.len() {
        let total_weight = discs[d].total_weight(&discs);
        discs[d].set_total_weight(total_weight);
    }

    // find unbalanced discs
    let mut unbalanced: Vec<&Disc> = discs.iter().filter(|disc| !disc.is_balanced(&discs)).collect();

    // get unbalanced disc with most parents
    unbalanced.sort_by_key(|disc| disc.count_parents(&discs));

    let disc_name = unbalanced.last().unwrap().name.clone();

    // get  children of found disc
    let mut children: Vec<&Disc> = discs.iter().filter(|disc| disc.parent == Some(disc_name.clone())).collect();

    // sort children by total weight to find max / min
    children.sort_by_key(|disc| disc.total_weight(&discs));

    let diff = children.last().unwrap().total_weight(&discs) - children.first().unwrap().total_weight(&discs);

    let result1 = discs.iter().find(|d| d.parent.is_none()).unwrap().name.clone();
    let result2 = children.last().unwrap().weight - diff;

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
    total_weight: i32,
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
            total_weight: 0,
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

    fn set_total_weight(&mut self, weight: i32)
    {
        self.total_weight = weight;
    }

    fn total_weight(&self, discs: &[Disc]) -> i32
    {
        if self.total_weight > 0 {
            return self.total_weight;
        }

        let mut weight = self.weight;

        weight += discs
            .iter()
            .filter(|disc| self.children.contains(&disc.name))
            .map(|disc| disc.total_weight(discs))
            .sum::<i32>();

        weight
    }

    fn is_balanced(&self, discs: &[Disc]) -> bool
    {
        discs
            .iter()
            .filter(|disc| disc.parent == Some(self.name.clone()))
            .map(|disc| disc.total_weight(discs))
            .collect::<std::collections::HashSet<i32>>()
            .len()
            <= 1
    }
}

impl std::str::FromStr for Disc
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        let mut parts = s.split(" -> ");

        let name_weight: Vec<&str> = parts.next().unwrap().split(' ').collect();

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
