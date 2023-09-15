use regex::Regex;

#[derive(Debug)]
struct Command
{
    action: String,
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

pub fn solve() -> (usize, i64)
{
    let reg = Regex::new(r"(.*) (\d+),(\d+) through (\d+),(\d+)").unwrap();

    let mut input = vec![];

    for (_, [action, x1, y1, x2, y2]) in reg.captures_iter(include_str!("input.txt")).map(|c| c.extract()) {
        input.push(Command {
            action: action.to_string(),
            x1: x1.parse().unwrap(),
            y1: y1.parse().unwrap(),
            x2: x2.parse().unwrap(),
            y2: y2.parse().unwrap(),
        });
    }

    let mut grid1 = vec![vec![false; 1000]; 1000];
    let mut grid2 = vec![vec![0; 1000]; 1000];

    for command in input {
        for x in command.x1..=command.x2 {
            for y in command.y1..=command.y2 {
                match command.action.as_str() {
                    "toggle" => {
                        grid1[x][y] = !grid1[x][y];
                        grid2[x][y] += 2;
                    }
                    "turn on" => {
                        grid1[x][y] = true;
                        grid2[x][y] += 1;
                    }
                    "turn off" => {
                        grid1[x][y] = false;
                        grid2[x][y] -= 1;

                        if grid2[x][y] < 0 {
                            grid2[x][y] = 0;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    let result1 = grid1.iter().flatten().filter(|&x| *x).count();
    let result2 = grid2.iter().flatten().sum::<i64>();

    println!("6\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

#[test]
fn test()
{
    assert_eq!(solve(), (543903, 14687245));
}
