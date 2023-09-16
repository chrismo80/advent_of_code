use crate::extensions::converter::Parser;
use crate::path_finding::grid::Grid;

pub fn solve() -> (usize, usize)
{
    let size = 50;
    let input = 1350;

    let mut map = "".to_string();

    for y in 0..size {
        for x in 0..size {
            let bit_mask = format!("{:b}", x * x + 3 * x + 2 * x * y + y + y * y + input);
            match bit_mask.chars().filter(|&c| c == '1').count() % 2 {
                0 => map += ".",
                _ => map += "#",
            }
        }
        map += "\n";
    }

    let grid = map.as_str().to_char_grid();

    let search = Grid::<char>::new(&grid, Box::new(|_, &next| next != '#'));
    let result1 = search.bfs((1, 1), (31, 39)).unwrap().len();

    let positions: Vec<(usize, usize)> = (0..size).flat_map(|y| (0..size).map(move |x| (x, y))).collect();
    let result2 = positions
        .iter()
        .filter(|pos| grid[pos.1][pos.0] != '#')
        .filter_map(|&pos| search.bfs((1, 1), pos))
        .filter(|path| path.len() <= 50)
        .count();

    println!("13\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

#[test]
fn test()
{
    assert_eq!(solve(), (92, 124));
}
