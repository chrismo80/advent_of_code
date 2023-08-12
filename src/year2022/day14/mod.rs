use std::collections::*;

pub fn solve() -> (usize, usize)
{
    let mut lines = include_str!("input.txt").lines().collect::<Vec<_>>();

    let walls = build_walls(&lines);
    let result1 = run_sand(&walls).len();

    lines.push("300,173 -> 700,173");

    let walls = build_walls(&lines);
    let result2 = run_sand(&walls).len();

    println!("14\t{result1}\t\t{result2}");

    (result1, result2)
}

fn run_sand(walls: &HashSet<(i32, i32)>) -> HashSet<(i32, i32)>
{
    let mut sand: HashSet<(i32, i32)> = HashSet::new();
    let mut unit = (500, 0);

    let bottom = walls.iter().map(|(_, y)| y).max().unwrap();
    let mut top = *bottom;

    while top > 0 && bottom > &unit.1 {
        unit.1 += 1;
        if !walls.contains(&unit) && !sand.contains(&unit) {
            continue;
        }

        unit.0 -= 1;
        if !walls.contains(&unit) && !sand.contains(&unit) {
            continue;
        }

        unit.0 += 2;
        if !walls.contains(&unit) && !sand.contains(&unit) {
            continue;
        }

        unit.0 -= 1;
        unit.1 -= 1;
        sand.insert(unit);

        top = top.min(unit.1);

        unit = (500, top - 1);
    }

    sand
}

fn build_walls(lines: &Vec<&str>) -> HashSet<(i32, i32)>
{
    let mut walls = HashSet::new();

    for row in lines {
        let points: Vec<(i32, i32)> = row
            .split(" -> ")
            .map(|point| {
                point
                    .split(',')
                    .map(|coord| coord.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .map(|point| (point[0], point[1]))
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
