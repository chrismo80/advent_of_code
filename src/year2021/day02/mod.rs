use crate::extensions::converter::Converter;

pub fn solve() -> (usize, usize)
{
    let input = include_str!("input.txt").to_vec_of_vec::<String>("\n", " ");

    let part1 = input.iter().fold((0, 0), |(mut hor, mut depth), current| {
        let value = current[1].parse::<usize>().unwrap();

        match current[0].as_str() {
            "forward" => hor += value,
            "down" => depth += value,
            "up" => depth -= value,
            _ => (),
        }

        (hor, depth)
    });

    let part2 = input.iter().fold((0, 0, 0), |(mut hor, mut depth, mut aim), current| {
        let value = current[1].parse::<usize>().unwrap();

        match current[0].as_str() {
            "forward" => {
                hor += value;
                depth += aim * value;
            }
            "down" => aim += value,
            "up" => aim -= value,
            _ => (),
        }

        (hor, depth, aim)
    });

    let result1 = part1.0 * part1.1;
    let result2 = part2.0 * part2.1;

    println!("2\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

#[test]
fn test()
{
    assert_eq!(solve(), (1840243, 1727785422));
}
