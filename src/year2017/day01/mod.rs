pub fn solve() -> (usize, usize)
{
    let input: Vec<u8> = include_str!("input.txt")
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();

    let result1 = find_matches(&input, 1);
    let result2 = find_matches(&input, input.len() / 2);

    println!("1\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

fn find_matches(input: &[u8], offset: usize) -> usize
{
    input
        .iter()
        .enumerate()
        .filter(|&(i, &c)| c == input[(i + offset) % input.len()])
        .map(|(_, &c)| c as usize)
        .sum()
}

#[test]
fn test()
{
    assert_eq!(solve(), (1158, 1132));
}
