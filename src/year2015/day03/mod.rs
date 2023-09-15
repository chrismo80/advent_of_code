use std::collections::HashSet;

pub fn solve() -> (usize, usize)
{
    let input: Vec<char> = include_str!("input.txt").chars().collect();

    let mut houses1 = vec![(0, 0)];
    let mut houses2 = vec![(0, 0)];

    for step in input.iter() {
        houses1.push(new_house(houses1.last().unwrap(), step));
    }

    for step in input.iter().step_by(2) {
        houses2.push(new_house(houses2.last().unwrap(), step));
    }

    houses2.push((0, 0));

    for step in input.iter().skip(1).step_by(2) {
        houses2.push(new_house(houses2.last().unwrap(), step));
    }

    let result1 = houses1.iter().collect::<HashSet<_>>().len();
    let result2 = houses2.iter().collect::<HashSet<_>>().len();

    println!("3\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

fn new_house(last_house: &(i32, i32), direction: &char) -> (i32, i32)
{
    match direction {
        '>' => (last_house.0, last_house.1 + 1),
        '<' => (last_house.0, last_house.1 - 1),
        'v' => (last_house.0 + 1, last_house.1),
        '^' => (last_house.0 - 1, last_house.1),
        _ => panic!("Invalid direction"),
    }
}

#[test]
fn test()
{
    assert_eq!(solve(), (2592, 2360));
}
