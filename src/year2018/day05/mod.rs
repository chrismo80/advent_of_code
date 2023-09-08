use regex::Regex;

pub fn solve() -> (usize, usize)
{
    let input = include_str!("input.txt");

    let reactions = (b'a'..=b'z')
        .map(|c| format!("{}{}", c as char, (c as char).to_uppercase()))
        .chain((b'A'..=b'Z').map(|c| format!("{}{}", c as char, (c as char).to_ascii_lowercase())))
        .collect::<Vec<String>>()
        .join("|");

    let regex = Regex::new(reactions.as_str()).unwrap();

    let result1 = react(input.to_string(), &regex);
    let result1 = polymere(input.chars().map(|c| c as u8));

    // let result2 = (b'a'..=b'z')
    //     .map(|c| react(input.replace([c as char, (c.to_ascii_uppercase()) as char], ""), &regex))
    //     .min()
    //     .unwrap();
    let result2 = (b'a'..=b'z')
        .map(|c| {
            polymere(
                input
                    .replace([c as char, (c.to_ascii_uppercase()) as char], "")
                    .chars()
                    .map(|c| c as u8),
            )
        })
        .min()
        .unwrap();

    println!("5\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

fn react(mut polymer: String, regex: &Regex) -> usize // slow
{
    let mut length = 0;

    while length != polymer.len() {
        length = polymer.len();
        polymer = regex.replace_all(polymer.as_str(), "").to_string();
    }

    polymer.len()
}

// https://www.reddit.com/r/adventofcode/comments/a3912m/comment/eb5h4qk
fn polymere(input: impl Iterator<Item = u8>) -> usize // fast
{
    let mut output: Vec<u8> = Vec::new();

    for ch in input {
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
