pub fn solve() -> (usize, usize)
{
    let input: Vec<&str> = include_str!("input.txt").lines().collect();

    let result1 = input
        .iter()
        .filter(|s| has_3_vowels(s) && has_chars_in_a_row(s) && has_no_invalid(s))
        .count();

    let result2 = input
        .iter()
        .filter(|s| contains_repeated_inbetween(s) && contains_multiple_pairs(s))
        .count();

    println!("5\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

fn contains_repeated_inbetween(word: &str) -> bool
{
    word.chars().zip(word.chars().skip(2)).any(|(a, b)| a == b)
}

fn contains_multiple_pairs(word: &str) -> bool
{
    let pairs: Vec<_> = word
        .chars()
        .zip(word.chars().skip(1))
        .map(|p| [p.0, p.1].iter().collect::<String>())
        .filter(|p| word.matches(p).count() > 1)
        .map(|p| word.match_indices(&p).collect::<Vec<_>>())
        .flat_map(|m| m.iter().map(|i| i.0).collect::<Vec<_>>())
        .collect();

    pairs.windows(2).any(|w| w[1] - w[0] > 1)
}

fn has_no_invalid(word: &str) -> bool
{
    let invalid = ["ab", "cd", "pq", "xy"];

    !invalid.iter().any(|s| word.contains(s))
}

fn has_chars_in_a_row(word: &str) -> bool
{
    word.chars().zip(word.chars().skip(1)).any(|(a, b)| a == b)
}

fn has_3_vowels(word: &str) -> bool
{
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    word.chars().map(|c| vowels.contains(&c) as i32).sum::<i32>() >= 3
}

#[test]
fn test()
{
    assert_eq!(solve(), (236, 51));
}
