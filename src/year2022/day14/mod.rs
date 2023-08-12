use std::collections::*;

pub fn solve() -> (usize, usize)
{
    let mut lines = include_str!("input.txt").lines().collect::<Vec<_>>();

    let walls = build_walls(&lines);
    let result1 = run_sand(&walls).len();

    //lines.push("485,11 -> 515,11");
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

    let bottom = *walls.iter().map(|(_, y)| y).max().unwrap();
    let mut top = bottom;

    while top > 0 && bottom > unit.1 {
        unit.1 += 1;
        if !sand.contains(&unit) && !walls.contains(&unit) {
            continue;
        }

        unit.0 -= 1;
        if !sand.contains(&unit) && !walls.contains(&unit) {
            continue;
        }

        unit.0 += 2;
        if !sand.contains(&unit) && !walls.contains(&unit) {
            continue;
        }

        unit.0 -= 1;
        unit.1 -= 1;
        sand.insert(unit);
        //print_map(walls, &sand);

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

fn print_map(walls: &HashSet<(i32, i32)>, sand: &HashSet<(i32, i32)>)
{
    let mut output = Vec::new();

    let min_x = walls.iter().map(|(x, _)| *x).min().unwrap_or(0);
    let max_x = walls.iter().map(|(x, _)| *x).max().unwrap_or(0);
    let max_y = walls.iter().map(|(_, y)| *y).max().unwrap_or(0);

    for y in 0..=max_y {
        let mut row = Vec::new();

        for x in min_x..=max_x {
            row.push(if walls.contains(&(x, y)) {
                '#'
            }
            else if sand.contains(&(x, y)) {
                'o'
            }
            else {
                '.'
            });
        }

        output.push(row.iter().collect::<String>());
    }

    for line in output {
        println!("{}", line);
    }

    std::thread::sleep(std::time::Duration::from_millis(100));
}
