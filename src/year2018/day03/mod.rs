use std::collections::HashSet;

pub fn solve() -> (usize, usize)
{
    let mut input: Vec<(Vec<usize>, HashSet<usize>)> = include_str!("input.txt")
        .lines()
        .map(|line| {
            line.split(['#', '@', ',', ':', 'x'].as_ref())
                .skip(1)
                .map(|e| e.trim().parse::<usize>().unwrap())
                .collect()
        })
        .map(|items| (items, HashSet::new()))
        .collect();

    let mut square_inches = [0; 1000 * 1000].to_vec();

    for claim in &mut input {
        for x in claim.0[1]..claim.0[1] + claim.0[3] {
            for y in claim.0[2]..claim.0[2] + claim.0[4] {
                let pos = x * 1000 + y;
                square_inches[pos] += 1;
                claim.1.insert(pos);
            }
        }
    }

    let result1 = square_inches.iter().filter(|e| **e > 1).count();
    let result2 = input
        .iter()
        .find(|claim| claim.1.iter().all(|pos| square_inches[*pos] == 1))
        .unwrap()
        .0[0];

    println!("3\t{:<20}\t{:<20}", result1, result2);

    (result1, result2)
}

#[test]
fn test()
{
    assert_eq!(solve(), (120419, 445));
}
