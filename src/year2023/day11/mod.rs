use crate::extensions::converter::Matrix;
use crate::extensions::converter::Parser;

pub fn solve() -> (usize, usize)
{
    let input = include_str!("input.txt").to_char_grid();

    let empty_rows = empty_space(&input);
    let empty_cols = empty_space(&input.transpose());

    let get_expansion = |empty: &[usize], high: &usize, low: &usize, expansion: usize| {
        high - low + empty.iter().filter(|&i| i > low && i < high).count() * (expansion - 1)
    };

    let distance = |a: (usize, usize), b: (usize, usize), expansion: usize| {
        let x = match a.0 < b.0 {
            true => get_expansion(&empty_cols, &b.0, &a.0, expansion),
            false => get_expansion(&empty_cols, &a.0, &b.0, expansion),
        };

        let y = match a.1 < b.1 {
            true => get_expansion(&empty_rows, &b.1, &a.1, expansion),
            false => get_expansion(&empty_rows, &a.1, &b.1, expansion),
        };

        x + y
    };

    let galaxies: Vec<(usize, usize)> = (0..input.len())
        .flat_map(|y| (0..input[0].len()).map(move |x| (x, y)))
        .filter(|&(x, y)| input[y][x] == '#')
        .collect();

    let mut result1 = 0;
    let mut result2 = 0;

    for (&from, &to) in galaxies
        .iter()
        .flat_map(|f| galaxies.iter().map(move |t| (f, t)))
        .filter(|(f, t)| f < t)
    {
        result1 += distance(from, to, 2);
        result2 += distance(from, to, 1_000_000);
    }

    println!("11\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

fn empty_space(space: &[Vec<char>]) -> Vec<usize>
{
    space
        .iter()
        .enumerate()
        .filter(|&(_, row)| row.iter().all(|&c| c == '.'))
        .map(|(i, _)| i)
        .collect()
}

#[test]
fn test()
{
    assert_eq!(solve(), (9769724, 603020563700));
}
