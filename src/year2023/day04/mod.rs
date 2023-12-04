use crate::extensions::converter::Parser;

pub fn solve() -> (usize, usize)
{
    let cards = include_str!("input.txt").to_vec::<Card>("\n");

    let mut counters = vec![1; cards.len()];

    for i in 0..counters.len() {
        for j in i..i + cards[i].matches {
            if j + 1 < counters.len() {
                counters[j + 1] += counters[i];
            }
        }
    }

    let result1 = cards.iter().map(|card| 2_usize.pow(card.matches as u32) / 2).sum();
    let result2 = counters.iter().sum();

    println!("4\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

impl std::str::FromStr for Card
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        let mut sets = s.split(':').next_back().unwrap().split('|');
        let wins = sets.next().unwrap().to_vec::<usize>(" ");
        let hand = sets.next().unwrap().to_vec::<usize>(" ");
        let matches = hand.iter().filter(|&n| wins.contains(n)).count();

        Ok(Card { wins, hand, matches })
    }
}

struct Card
{
    wins: Vec<usize>,
    hand: Vec<usize>,
    matches: usize,
}

#[test]
fn test()
{
    assert_eq!(solve(), (23847, 8570000));
}
