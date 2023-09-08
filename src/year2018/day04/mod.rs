use std::collections::HashMap;

use crate::extensions::converter::Converter;

pub fn solve() -> (usize, usize)
{
    let mut input = include_str!("input.txt").to_vec::<String>("\n");

    input.sort();

    let mut guards = HashMap::new();
    let mut guard = 0;
    let mut start = 0;

    for line in input {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let minute = parts[1][3..5].parse::<usize>().unwrap();

        match parts[2] {
            "Guard" => {
                guard = parts[3][1..].parse::<usize>().unwrap();
                guards.entry(guard).or_insert_with(|| vec![0; 60]);
            }
            "falls" => start = minute,
            "wakes" => (start..minute).for_each(|i| guards.get_mut(&guard).unwrap()[i] += 1),
            _ => panic!("Unknown input"),
        }
    }

    let guard1 = guards.iter().max_by_key(|(_, v)| v.iter().sum::<usize>()).unwrap();
    let guard2 = guards.iter().max_by_key(|(_, v)| v.iter().max().unwrap()).unwrap();

    let best_minute = |guard: &[usize]| guard.iter().position(|m| m == guard.iter().max().unwrap()).unwrap();

    let result1 = guard1.0 * best_minute(guard1.1);
    let result2 = guard2.0 * best_minute(guard2.1);

    println!("4\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

#[test]
fn test()
{
    assert_eq!(solve(), (36898, 80711));
}
