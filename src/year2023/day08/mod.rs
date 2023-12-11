use crate::extensions::converter::Parser;
use crate::extensions::math::Primes;

pub fn solve() -> (usize, usize)
{
    let input = include_str!("input.txt").to_vec::<String>("\n\n");

    let instructions: Vec<char> = input.first().unwrap().chars().collect();

    let network = input.last().unwrap().as_str();
    let map = network.to_vec_from_regex(r"^(\w{3}) = \((\w{3})\, (\w{3})\)$");

    let starts: Vec<&str> = map.iter().filter(|m| m[0].ends_with('A')).map(|m| m[0]).collect();
    let distances: Vec<usize> = starts.iter().map(|start| escape(&map, &instructions, start, "Z")).collect();

    let result1 = escape(&map, &instructions, "AAA", "ZZZ");
    let result2 = distances.lcm();

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

#[test]
fn test()
{
    assert_eq!(solve(), (16531, 24035773251517));
}
