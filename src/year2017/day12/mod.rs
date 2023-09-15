pub fn solve() -> (usize, usize)
{
    let input: Vec<Vec<usize>> = include_str!("input.txt")
        .lines()
        .map(|l| l.split([' ', ',']).filter_map(|e| e.parse().ok()).collect())
        .collect();

    let mut done = vec![false; input.len()];
    let mut group = Vec::new();
    let mut groups = Vec::new();
    let mut count;

    while done.iter().any(|p| !p) {
        count = 0;
        group.push(done.iter().position(|p| !p).unwrap());

        while let Some(i) = group.pop() {
            if !done[i] {
                done[i] = true;
                count += 1;
                group.extend(input[i].iter().filter(|&p| !done[*p]));
            }
        }

        groups.push(count);
    }

    let result1 = *groups.first().unwrap();
    let result2 = groups.len();

    println!("12\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

#[test]
fn test()
{
    assert_eq!(solve(), (115, 221));
}
