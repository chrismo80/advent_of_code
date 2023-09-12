use std::collections::*;

pub fn solve() -> (String, i64)
{
    let input: Vec<(&str, i64, Option<&str>)> = include_str!("input.txt")
        .lines()
        .map(|l| {
            (
                l.split(' ').next().unwrap(),
                l.split(['(', ')']).nth(1).unwrap().parse().unwrap(),
                l.split(" -> ").nth(1),
            )
        })
        .collect();

    let mut discs = HashMap::new();

    for line in &input {
        if let Some(children) = line.2 {
            discs.insert(line.0, (line.1, children.split(", ").collect()));
        }
        else {
            discs.insert(line.0, (line.1, Vec::new()));
        }
    }

    let mut unbalanced: Vec<(usize, &str)> = discs
        .keys()
        .filter(|name| !balanced(&discs, name))
        .map(|&name| (get_parents_count(&discs, name), name))
        .collect();

    unbalanced.sort();

    let mut last_children: Vec<(i64, &str)> = discs[unbalanced.last().unwrap().1]
        .1
        .iter()
        .map(|&child| (total_weight(&discs, child), child))
        .collect();

    last_children.sort();

    let diff = last_children.last().unwrap().0 - last_children.first().unwrap().0;

    let result1 = discs
        .keys()
        .find(|name| get_parents_count(&discs, name) == 0)
        .unwrap()
        .to_string();
    let result2 = discs[last_children.last().unwrap().1].0 - diff;

    println!("7\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

fn balanced(discs: &HashMap<&str, (i64, Vec<&str>)>, name: &str) -> bool
{
    if discs[name].1.is_empty() {
        return true;
    }

    let mut weights = Vec::new();

    for child in &discs[name].1 {
        weights.push(total_weight(discs, child));
    }

    weights.iter().all(|w| w == &weights[0])
}

fn total_weight(discs: &HashMap<&str, (i64, Vec<&str>)>, name: &str) -> i64
{
    let mut weight = discs[name].0;

    for child in &discs[name].1 {
        weight += total_weight(discs, child);
    }

    weight
}

fn get_parents_count(discs: &HashMap<&str, (i64, Vec<&str>)>, name: &str) -> usize
{
    let mut parents = 0;

    if let Some(parent) = discs.iter().find(|&disc| disc.1 .1.contains(&name)) {
        parents += 1 + get_parents_count(discs, parent.0);
    }

    parents
}

#[test]
fn test()
{
    assert_eq!(solve(), ("cyrupz".to_string(), 193));
}
