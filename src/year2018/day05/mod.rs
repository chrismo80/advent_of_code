use rayon::prelude::*;
use regex::Regex;

pub fn solve() -> (usize, usize)
{
    let input = include_str!("input.txt");

    let result1 = react(input.to_string());

    let result2 = (b'a'..=b'z')
        .into_par_iter()
        .map(|c| react(input.replace([c as char, (c.to_ascii_uppercase()) as char], "")))
        .min()
        .unwrap();

    println!("5\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

fn react(mut polymer: String) -> usize // slow
{
    let reactions = (b'a'..=b'z')
        .map(|c| format!("{}{}", c as char, (c as char).to_uppercase()))
        .chain((b'A'..=b'Z').map(|c| format!("{}{}", c as char, (c as char).to_ascii_lowercase())))
        .collect::<Vec<String>>()
        .join("|");

    let regex = Regex::new(reactions.as_str()).unwrap();

    let mut length = 0;

    while length != polymer.len() {
        length = polymer.len();
        polymer = regex.replace_all(polymer.as_str(), "").to_string();
    }

    polymer.len()
}

// https://www.reddit.com/r/adventofcode/comments/a3912m/comment/eb5h4qk
// A is 0x41, a is 0x61
// B is 0x42, b is 0x62
// etc.
//
// For units to pass the comparison, their "letter part" must be equal,
// and their "case part" mut be different.
//
// Using the result of the bitwise XOR:
//
//        vvvv- same letter
// 0b0010_0000
//   ^^^^------ not the same case
//
// This is much faster than using the `to_lowercase` function, since Rust's
// awesome UTF-8 support uses a big conversion table, *and* needs to support
// multiple-characters lowercase.
// (left as u8) ^ (right as u8) == 0b0010_0000
fn polymere(input: String) -> usize // fast
{
    let mut output: Vec<u8> = Vec::new();

    for ch in input.chars().map(|c| c as u8) {
        if output.last().map(|&l| l ^ ch == 32).unwrap_or(false) {
            output.pop();
        }
        else {
            output.push(ch);
        }
    }

    output.len()
}

#[test]
fn test()
{
    assert_eq!(solve(), (11042, 6872));
}
