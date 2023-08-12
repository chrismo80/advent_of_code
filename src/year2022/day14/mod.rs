use std::collections::*;

pub fn solve() -> (usize, usize)
{
    let mut lines = include_str!("input.txt").lines().collect();

    let walls = build_walls(&lines);
    let result1 = run_sand(&walls);

    //lines.push("485,11 -> 515,11"); // test.txt
    lines.push("300,173 -> 700,173");

    let walls = build_walls(&lines);
    let result2 = run_sand(&walls);

    println!("14\t{result1}\t\t{result2}");

    (result1, result2)
}

fn run_sand(walls: &HashSet<(usize, usize)>) -> usize
{
    let mut grid = vec![vec![false; 1000]; 1000];

    for (x, y) in walls {
        grid[*x][*y] = true;
    }

    let bottom = *walls.iter().map(|(_, y)| y).max().unwrap();
    let mut top = bottom;

    let (mut x, mut y) = (500, 0);

    while bottom > y {
        y += 1; // 1 down
        if !grid[x][y] {
            continue;
        }

        x -= 1; // 1 left
        if !grid[x][y] {
            continue;
        }

        x += 2; // 1 right
        if !grid[x][y] {
            continue;
        }

        x -= 1; // go back
        y -= 1;
        grid[x][y] = true;

        if y == 0 {
            break;
        }

        top = top.min(y);

        x = 500;
        y = top - 1;
    }

    let mut count = 0;

    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if grid[x][y] {
                count += 1;
            }
        }
    }

    count - walls.len()
}

fn build_walls(lines: &Vec<&str>) -> HashSet<(usize, usize)>
{
    let mut walls = HashSet::new();

    for row in lines {
        let points: Vec<(usize, usize)> = row
            .split(" -> ")
            .map(|point| {
                let coords: Vec<usize> = point.split(',').map(|coord| coord.parse().unwrap()).collect();
                (coords[0], coords[1])
            })
            .collect();

        for i in 1..points.len() {
            for x in points[i].0.min(points[i - 1].0)..=points[i].0.max(points[i - 1].0) {
                for y in points[i].1.min(points[i - 1].1)..=points[i].1.max(points[i - 1].1) {
                    walls.insert((x, y));
                }
            }
        }
    }

    walls
}

#[cfg(test)]
mod tests
{
    #[test]
    fn solve()
    {
        assert_eq!(super::solve(), (1068, 27936));
    }
}
