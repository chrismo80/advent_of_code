use crate::extensions::converter::Parser;

pub fn solve() -> (usize, usize)
{
    let matches: Vec<usize> = include_str!("input.txt").lines().map(get_matches).collect();

    let mut counts = vec![1; matches.len()];

    for m in 0..counts.len() {
        for i in m..m + matches[m] {
            if i + 1 < counts.len() {
                counts[i + 1] += counts[m];
            }
        }
    }

    let result1 = matches.iter().map(|&m| 2_usize.pow(m as u32) / 2).sum();
    let result2 = counts.iter().sum();

    println!("4\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

fn get_matches(card: &str) -> usize
{
    let mut sets = card.split(':').next_back().unwrap().split('|');

    let wins = sets.next().unwrap().to_vec::<usize>(" ");
    let hand = sets.next().unwrap().to_vec::<usize>(" ");

    hand.iter().filter(|&n| wins.contains(n)).count()
}

#[test]
fn test()
{
    assert_eq!(solve(), (23847, 8570000));
}
