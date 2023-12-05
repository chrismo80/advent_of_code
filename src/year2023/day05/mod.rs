use crate::extensions::converter::Parser;

pub fn solve() -> (i64, i64)
{
    let input = include_str!("input.txt").to_vec::<String>("\n\n");

    let mut input = input.iter();
    let mut mappings = Vec::new();

    let parse = |s: &String| s.split(':').next_back().unwrap().to_vec_of_vec::<i64>("\n", " ");

    let seeds = input.next().unwrap().split(':').next_back().unwrap().to_vec::<i64>(" ");

    while input.len() > 0 {
        mappings.push(parse(input.next().unwrap()));
    }

    let get_location = |mut seed: i64| {
        mappings.iter().for_each(|mapping| {
            seed = match mapping.iter().find(|m| seed >= m[1] && seed <= m[1] + m[2]) {
                Some(m) => seed + m[0] - m[1],
                None => seed,
            }
        });
        seed
    };

    let mut mins = Vec::new();

    for i in (0..seeds.len()).step_by(2) {
        mins.push((seeds[i]..seeds[i] + seeds[i + 1]).map(get_location).min().unwrap());
    }

    let result1 = seeds.iter().map(|&s| get_location(s)).min().unwrap();
    let result2 = *mins.iter().min().unwrap();

    println!("5\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

#[test]
fn test()
{
    assert_eq!(solve(), (26273516, 34039469));
}
