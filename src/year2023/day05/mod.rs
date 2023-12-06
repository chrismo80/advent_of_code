use crate::extensions::converter::Parser;

pub fn solve() -> (i64, i64)
{
    let input = include_str!("input.txt").to_vec::<String>("\n\n");

    let mut input = input.iter();

    let seeds = input.next().unwrap().split(':').next_back().unwrap().to_vec::<i64>(" ");

    let parse = |s: &String| s.split(':').next_back().unwrap().to_vec_of_vec::<i64>("\n", " ");

    let mut mappings = Vec::new();

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

    let result1 = seeds.iter().map(|&s| get_location(s)).min().unwrap();

    let get_min = |start: i64, length: i64, step: usize| {
        let locations: Vec<i64> = (start..start + length).step_by(step).map(get_location).collect();
        let min = locations.iter().min().unwrap();

        (start + (step * locations.iter().position(|l| l == min).unwrap()) as i64, *min)
    };

    let mut best_seed = 0;
    let mut best_min = i64::MAX;

    for i in (0..seeds.len()).step_by(2) {
        let range_min = get_min(seeds[i], seeds[i + 1], 10000);

        if range_min.1 < best_min {
            best_seed = range_min.0;
            best_min = range_min.1;
        }
    }

    let result2 = get_min(best_seed - 10000, 20000, 1).1;

    println!("5\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

#[test]
fn test()
{
    assert_eq!(solve(), (26273516, 34039469));
}
