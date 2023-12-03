use std::collections::hash_map::Entry;
use std::collections::HashMap;

use crate::extensions::converter::Parser;

pub fn solve() -> (usize, usize)
{
    let input = include_str!("input.txt").to_char_grid();

    let parts: Vec<Number> = get_numbers(&input).iter().filter(|n| n.symbol.is_some()).cloned().collect();

    let result1 = parts.iter().map(|p| p.value).sum();

    let gears: Vec<Number> = parts.iter().filter(|n| n.symbol.unwrap().0 == '*').cloned().collect();

    let mut gear_ratios: HashMap<Option<(char, usize, usize)>, Vec<usize>> = HashMap::new();

    for gear in &gears {
        if let Entry::Vacant(e) = gear_ratios.entry(gear.symbol) {
            e.insert(vec![gear.value]);
        }
        else {
            gear_ratios.get_mut(&gear.symbol).unwrap().push(gear.value);
        }
    }

    let result2 = gear_ratios
        .iter()
        .filter(|g| g.1.len() > 1)
        .map(|g| g.1.iter().product::<usize>())
        .sum::<usize>();

    println!("3\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

fn get_numbers(input: &[Vec<char>]) -> Vec<Number>
{
    let mut numbers = Vec::new();

    for (row, line) in input.iter().enumerate() {
        let mut checked = 0;

        for (pos, character) in line.iter().enumerate() {
            if character.is_ascii_digit() && pos >= checked {
                let mut length = 1;

                while pos + length < line.len() && line[pos + length].is_ascii_digit() {
                    length += 1;
                }
                checked = pos + length;

                numbers.push(Number {
                    value: line[pos..pos + length].iter().collect::<String>().parse::<usize>().unwrap(),
                    symbol: get_symbol(input, row, pos, length),
                });
            }
        }
    }

    numbers
}

fn get_symbol(map: &[Vec<char>], row: usize, pos: usize, length: usize) -> Option<(char, usize, usize)>
{
    let mut neighbors = Vec::new();

    for r in row as i64 - 1..row as i64 + 2 {
        for c in pos as i64 - 1..pos as i64 + length as i64 + 1 {
            neighbors.push((r, c));
        }
    }

    neighbors
        .iter()
        .filter(|p| !(p.0 == row as i64 && p.1 >= pos as i64 && p.1 < pos as i64 + length as i64))
        .filter(|p| p.0 >= 0 && p.0 < map.len() as i64)
        .filter(|p| p.1 >= 0 && p.1 < map[0].len() as i64)
        .map(|p| (map[p.0 as usize][p.1 as usize], p.0 as usize, p.1 as usize))
        .find(|&s| !s.0.is_ascii_digit() && s.0 != '.')
}

#[derive(Debug, PartialEq, Clone)]
struct Number
{
    value: usize,
    symbol: Option<(char, usize, usize)>,
}

#[test]
fn test()
{
    assert_eq!(solve(), (521515, 69527306));
}
