use std::collections::*;

pub fn solve() -> (usize, usize)
{
    let mut lines = include_str!("input.txt").lines().collect::<Vec<_>>();

    let walls = build_walls(&lines);
    let result1 = run_sand(&walls);

    //lines.push("485,11 -> 515,11");
    lines.push("300,173 -> 700,173");

    let walls = build_walls(&lines);
    let result2 = run_sand(&walls);

    println!("14\t{result1}\t\t{result2}");

    (result1, result2)
}

fn run_sand(walls: &HashSet<(i32, i32)>) -> usize
{
    let mut grid = vec![vec![false; 1000]; 1000];

    for (x, y) in walls {
        grid[*x as usize][*y as usize] = true;
    }

    let bottom = *walls.iter().map(|(_, y)| y).max().unwrap();
    let mut top = bottom;

    let mut unit = (500, 0);

    while top > 0 && bottom > unit.1 {
        unit.1 += 1;
        if !grid[unit.0 as usize][unit.1 as usize] {
            continue;
        }

        unit.0 -= 1;
        if !grid[unit.0 as usize][unit.1 as usize] {
            continue;
        }

        unit.0 += 2;
        if !grid[unit.0 as usize][unit.1 as usize] {
            continue;
        }

        unit.0 -= 1;
        unit.1 -= 1;
        grid[unit.0 as usize][unit.1 as usize] = true;

        top = top.min(unit.1);

        unit = (500, top - 1);
    }

    let mut sand = HashSet::new();
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if grid[x][y] {
                sand.insert((x as i32, y as i32));
            }
        }
    }

    sand.len() - walls.len()
}

fn build_walls(lines: &Vec<&str>) -> HashSet<(i32, i32)>
{
    let mut walls = HashSet::new();

    for row in lines {
        let points: Vec<(i32, i32)> = row
            .split(" -> ")
            .map(|point| {
                let coords: Vec<i32> = point.split(',').map(|coord| coord.parse().unwrap()).collect();
                (coords[0], coords[1])
            })
            .collect();

        for i in 1..points.len() {
            let x_range = std::cmp::min(points[i].0, points[i - 1].0)..=std::cmp::max(points[i].0, points[i - 1].0);
            let y_range = std::cmp::min(points[i].1, points[i - 1].1)..=std::cmp::max(points[i].1, points[i - 1].1);

            for x in x_range {
                for y in y_range.clone() {
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
