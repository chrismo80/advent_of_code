pub fn solve() -> (usize, usize)
{
    let input: Vec<Vec<usize>> = include_str!("input.txt")
        .lines()
        .map(|l| l.split([' ', ',']).filter_map(|e| e.parse().ok()).collect())
        .collect();

    let mut groups = vec![0; input.len()];
    let mut active = Vec::new();
    let mut current = 0;

    while groups.iter().any(|&g| g == 0) {
        active.push(groups.iter().position(|&g| g == 0).unwrap());
        current += 1;

        while let Some(p) = active.pop() {
            if groups[p] == 0 {
                groups[p] = current;
                active.extend(input[p].iter().filter(|&g| groups[*g] == 0));
            }
        }
    }

    let result1 = groups.iter().filter(|&g| g == &1).count();
    let result2 = *groups.iter().max().unwrap();

    println!("12\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

#[test]
fn test()
{
    assert_eq!(solve(), (115, 221));
}
