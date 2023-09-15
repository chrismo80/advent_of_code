use std::collections::HashMap;

pub fn solve() -> (String, String)
{
    let input: Vec<&str> = include_str!("input.txt").lines().collect();

    let transposed: Vec<String> = (0..input[0].len())
        .map(|i| input.iter().map(|s| s.chars().nth(i).unwrap()).collect())
        .collect();

    let most_common = transposed
        .iter()
        .map(|word| {
            word.chars()
                .fold(HashMap::<char, usize>::new(), |mut map, element| {
                    *map.entry(element).or_default() += 1;
                    map
                })
                .into_iter()
                .max_by_key(|(_, value)| *value)
                .map(|(key, _)| key)
                .unwrap()
        })
        .collect::<String>();

    let least_common = transposed
        .iter()
        .map(|word| {
            word.chars()
                .fold(HashMap::<char, usize>::new(), |mut map, element| {
                    *map.entry(element).or_default() += 1;
                    map
                })
                .into_iter()
                .min_by_key(|(_, value)| *value)
                .map(|(key, _)| key)
                .unwrap()
        })
        .collect::<String>();

    let result1 = most_common;
    let result2 = least_common;

    println!("6\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

#[test]
fn test()
{
    assert_eq!(solve(), (String::from("ygjzvzib"), String::from("pdesmnoz")));
}
