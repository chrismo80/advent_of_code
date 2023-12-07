use crate::extensions::converter::Parser;
use crate::extensions::count_items::CountItems;

pub fn solve() -> (usize, usize)
{
    let input = include_str!("input.txt").to_vec_of_vec::<String>("\n", " ");

    let result = |cards: Vec<(HandType, String, usize)>| cards.iter().enumerate().map(|hand| (1 + hand.0) * hand.1 .2).sum();

    let mut hands: Vec<_> = input
        .iter()
        .map(|l| (rename(l.first().unwrap(), "B"), l.last().unwrap().parse::<usize>().unwrap()))
        .map(|(hand, bid)| (get_type(&hand.chars().collect()), hand, bid))
        .collect();

    hands.sort();

    let mut joker_hands: Vec<_> = input
        .iter()
        .map(|l| (l.first().unwrap(), l.last().unwrap().parse::<usize>().unwrap()))
        .map(|(hand, bid)| (get_type(&joker(rename(hand, "B")).chars().collect()), rename(hand, "1"), bid))
        .collect();

    joker_hands.sort();

    let result1 = result(hands);
    let result2 = result(joker_hands);

    println!("7\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

fn joker(hand: String) -> String
{
    if !hand.contains('B') {
        return hand;
    }

    if hand == "BBBBB" {
        return String::from("EEEEE");
    }

    let jokers: Vec<char> = hand.chars().filter(|&c| c != 'B').collect();

    let mut counts: Vec<(i32, char)> = jokers.count_items().iter().map(|c| (*c.1, *c.0)).collect();

    counts.sort();

    hand.replace('B', counts.last().unwrap().1.to_string().as_str())
}

fn rename(hand: &str, j: &str) -> String
{
    hand.replace('A', "E")
        .replace('K', "D")
        .replace('Q', "C")
        .replace('J', j)
        .replace('T', "A")
}

fn get_type(hand: &Vec<char>) -> HandType
{
    let cards = hand.count_items();

    match cards.len() {
        1 => HandType::FiveOfAKind,
        2 if cards.values().any(|&x| x == 4) => HandType::FourOfAKind,
        2 if cards.values().any(|&x| x == 3) => HandType::FullHouse,
        3 if cards.values().any(|&x| x == 3) => HandType::ThreeOfAKind,
        3 if cards.values().any(|&x| x == 2) => HandType::TwoPairs,
        4 => HandType::OnePair,
        _ => HandType::HighCard,
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType
{
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[test]
fn test()
{
    assert_eq!(solve(), (248559379, 249631254));
}
