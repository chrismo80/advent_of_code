pub fn solve() -> (usize, usize)
{
    let mut discs: Vec<Vec<usize>> = include_str!("input.txt")
        .lines()
        .map(|line| {
            line.split([' ', '#', '.'])
                .filter_map(|word| word.parse().ok())
                .collect::<Vec<usize>>()
        })
        .collect();

    let result1 = find_wait_time(&discs);

    discs.push(vec![discs.len() + 1, 11, 0]);

    let result2 = find_wait_time(&discs);

    println!("15\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

fn find_wait_time(discs: &[Vec<usize>]) -> usize
{
    (0..usize::MAX)
        .find(|&delay| discs.iter().all(|d| (d[0] + d[2] + delay) % d[1] == 0))
        .unwrap()
}

#[test]
fn test()
{
    assert_eq!(solve(), (122318, 3208583));
}
