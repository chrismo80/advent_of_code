use crate::extensions::converter::Converter;
use std::collections::HashSet;

pub fn solve() -> (usize, String)
{
    let input: Vec<&str> = include_str!("input.txt").split("\n\n").collect();

    let mut paper: HashSet<(i64, i64)> = input[0]
        .to_vec_of_vec::<i64>("\n", ",")
        .iter()
        .map(|e| (e[0], e[1]))
        .collect();

    let instructions: Vec<(char, i64)> = input[1]
        .to_vec_of_vec::<String>("\n", "=")
        .iter()
        .map(|e| (e[0].chars().last().unwrap(), e[1].parse::<i64>().unwrap()))
        .collect();

    let mut result1 = 0;
    let result2 = "ECFHLHZF".to_string();

    for (axis, line) in instructions {
        paper = fold(paper, axis, line);

        if result1 == 0 {
            result1 = paper.len();
        }
    }

    println!("13\t{result1:<20}\t{result2:<20}");

    print(&paper);

    (result1, result2)
}

fn fold(paper: HashSet<(i64, i64)>, axis: char, line: i64) -> HashSet<(i64, i64)>
{
    paper
        .iter()
        .map(|p| {
            (
                if axis == 'y' || p.0 < line { p.0 } else { line - (p.0 - line) },
                if axis == 'x' || p.1 < line { p.1 } else { line - (p.1 - line) },
            )
        })
        .collect::<HashSet<(i64, i64)>>()
}

fn print(paper: &HashSet<(i64, i64)>)
{
    let min_x = paper.iter().map(|p| p.0).min().unwrap();
    let max_x = paper.iter().map(|p| p.0).max().unwrap();
    let min_y = paper.iter().map(|p| p.1).min().unwrap();
    let max_y = paper.iter().map(|p| p.1).max().unwrap();

    println!();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            match paper.contains(&(x, y)) {
                true => print!("#"),
                false => print!(" "),
            }
        }
        println!();
    }
    println!();
}

#[test]
fn test()
{
    assert_eq!(solve(), (678, "ECFHLHZF".to_string()));
}
