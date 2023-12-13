use crate::extensions::converter::Matrix;
use crate::extensions::converter::Parser;

pub fn solve() -> (usize, usize)
{
    let input: Vec<&str> = include_str!("input.txt").split("\n\n").collect();

    let result1 = input.iter().map(|&p| calculate(p.to_char_grid())).sum::<usize>();
    let result2 = input.iter().map(|&p| fix_smudge(p.to_char_grid())).sum::<usize>();

    println!("13\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

fn fix_smudge(input: Vec<Vec<char>>) -> usize
{
    let mirror_row = find_mirror(&input, None);
    let mirror_col = find_mirror(&input.transpose(), None);

    for row in 0..input.len() {
        for col in 0..input[0].len() {
            let test = check_smudge(&input, row, col);

            if let Some(r) = find_mirror(&test, mirror_row) {
                return 100 * r;
            }

            if let Some(c) = find_mirror(&test.transpose(), mirror_col) {
                return c;
            }
        }
    }

    panic!("No solution found");
}

fn check_smudge(input: &[Vec<char>], row: usize, col: usize) -> Vec<Vec<char>>
{
    let mut result = input.to_vec();

    result[row][col] = match result[row][col] {
        '#' => '.',
        '.' => '#',
        _ => panic!("Invalid tile"),
    };

    result
}

fn calculate(rows: Vec<Vec<char>>) -> usize
{
    100 * find_mirror(&rows, None).unwrap_or(0) + find_mirror(&rows.transpose(), None).unwrap_or(0)
}

fn find_mirror(input: &[Vec<char>], skip: Option<usize>) -> Option<usize>
{
    (1..input.len())
        .filter(|&i| skip.is_none() || i != skip.unwrap())
        .find(|&i| is_mirrored(input, i))
}

fn is_mirrored(input: &[Vec<char>], row: usize) -> bool
{
    let mut upper = input.iter().take(row).rev().collect::<Vec<_>>();
    let mut lower = input.iter().skip(row).collect::<Vec<_>>();

    let length = upper.len().min(lower.len());

    upper = upper.iter().take(length).copied().collect::<Vec<_>>();
    lower = lower.iter().take(length).copied().collect::<Vec<_>>();

    upper == lower
}

#[test]
fn test()
{
    assert_eq!(solve(), (34821, 36919));
}
