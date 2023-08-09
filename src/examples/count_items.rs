use crate::extensions::count_items::*;

pub fn main()
{
    let words: Vec<String> = "Hello world wonderful world"
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    let numbers = vec![1, 2, 3, 4, 3, 2, 7, 2, 1, 3, 2, 1, 2, 3, 4, 3, 4, 5, 4, 3, 9];

    let map = words.count_items();
    let counts = numbers.count_items();

    println!("map: {:?}", map);
    counts.iter().for_each(|(key, value)| println!("{key}: {value}"));
}
