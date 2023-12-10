use crate::extensions::converter::Parser;

pub fn solve() -> (usize, usize)
{
    let input = include_str!("input.txt").to_vec::<String>("\n\n");

    let instructions: Vec<char> = input.first().unwrap().chars().collect();

    let network = input.last().unwrap().as_str();
    let map = network.to_vec_from_regex(r"^(\w{3}) = \((\w{3})\, (\w{3})\)$");

    let starts: Vec<&str> = map.iter().filter(|m| m[0].ends_with('A')).map(|m| m[0]).collect();
    let distances: Vec<usize> = starts.iter().map(|start| escape(&map, &instructions, start, "Z")).collect();

    let result1 = escape(&map, &instructions, "AAA", "ZZZ");
    let result2 = lcm(&distances);

    println!("8\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

fn escape(map: &[Vec<&str>], instructions: &[char], start: &str, end: &str) -> usize
{
    let mut distance = 0;
    let mut location = start;
    let mut position = map.iter().position(|m| m[0] == location).unwrap();

    while !location.ends_with(end) {
        go(map, instructions, &mut distance, &mut position, &mut location);
    }

    distance
}

fn go<'a>(map: &[Vec<&'a str>], instructions: &[char], distance: &mut usize, position: &mut usize, location: &mut &'a str)
{
    let direction = instructions[*distance % instructions.len()];

    *location = match direction {
        'L' => map[*position][1],
        'R' => map[*position][2],
        _ => panic!("Invalid direction"),
    };

    *position = map.iter().position(|m| m[0] == *location).unwrap();
    *distance += 1;
}

fn lcm(values: &[usize]) -> usize
{
    let gcf = gcf(values);

    gcf * values.iter().map(|value| value / gcf).product::<usize>()
}

fn gcf(values: &[usize]) -> usize
{
    let mut gcf = values[0];

    for value in values {
        gcf = find_gcf(gcf, *value);

        if gcf == 1 {
            return 1;
        }
    }

    gcf
}

fn find_gcf(a: usize, b: usize) -> usize
{
    if a == 0 {
        return b;
    }

    find_gcf(b % a, a)
}

#[test]
fn test()
{
    assert_eq!(solve(), (16531, 24035773251517));
}
