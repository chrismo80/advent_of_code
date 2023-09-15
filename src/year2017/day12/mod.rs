use std::collections::HashSet;

pub fn solve() -> (usize, usize)
{
    let input: Vec<Vec<usize>> = include_str!("input.txt")
        .lines()
        .map(|l| l.split([' ', ',']).filter_map(|e| e.parse().ok()).collect())
        .collect();

    let all: HashSet<usize> = HashSet::from_iter(input.iter().flatten().cloned());
    let mut found: HashSet<usize> = HashSet::new();

    let mut count = 0;
    let mut groups = 0;
    let mut result1 = 0;

    while found.len() < all.len() {
        let start = match found.len() {
            0 => 0,
            _ => *all.difference(&found).next().unwrap(),
        };

        found.insert(start);

        while count != found.len() {
            count = found.len();

            let to_search: Vec<&Vec<usize>> = input.iter().filter(|&p| !found.contains(p.first().unwrap())).collect();

            for connections in to_search.iter() {
                let h = HashSet::from_iter(connections.iter().copied());

                if found.intersection(&h).count() > 0 {
                    found = found.union(&h).cloned().collect();
                }
            }
        }

        groups += 1;

        if result1 == 0 {
            result1 = found.len();
        }
    }

    let result2 = groups;

    println!("12\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

#[test]
fn test()
{
    assert_eq!(solve(), (115, 221));
}
