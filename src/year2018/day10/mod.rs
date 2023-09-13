pub fn solve() -> (String, i64)
{
    let input: Vec<Vec<i64>> = include_str!("input.txt")
        .lines()
        .map(|l| l.split([',', ' ', '<', '>']).filter_map(|e| e.parse::<i64>().ok()).collect())
        .collect();

    let mut grid: Vec<(i64, i64)> = input.iter().map(|v| (v[0], v[1])).collect();
    let velos: Vec<(i64, i64)> = input.iter().map(|v| (v[2], v[3])).collect();

    let mut seconds = 0;

    while grid.iter().map(|g| g.1).max().unwrap() - grid.iter().map(|g| g.1).min().unwrap() > 10 {
        for i in 0..velos.len() {
            grid[i].0 += velos[i].0;
            grid[i].1 += velos[i].1;
        }
        seconds += 1;
    }

    let result1 = "PPNJEENH".to_string();
    let result2 = seconds;

    println!("10\t{result1:<20}\t{result2:<20}");

    print(&grid);

    (result1, result2)
}

fn print(grid: &[(i64, i64)])
{
    let min_x = grid.iter().map(|g| g.0).min().unwrap();
    let max_x = grid.iter().map(|g| g.0).max().unwrap();
    let min_y = grid.iter().map(|g| g.1).min().unwrap();
    let max_y = grid.iter().map(|g| g.1).max().unwrap();

    println!();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if grid.contains(&(x, y)) {
                print!("#");
            }
            else {
                print!(" ");
            }
        }
        println!();
    }
    println!();
}

#[test]
fn test()
{
    assert_eq!(solve(), ("PPNJEENH".to_string(), 10375));
}
