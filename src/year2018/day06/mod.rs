use crate::extensions::converter::Converter;
use std::{cmp::Ordering, collections::*};

pub fn solve() -> (usize, usize)
{
    let input = include_str!("input.txt").to_vec_of_vec::<i32>("\n", ", ");

    let get_total_distance_to = |x: i32, y: i32| -> i32 { input.iter().map(|p| (p[0] - x).abs() + (p[1] - y).abs()).sum() };
    let get_closest_index_to = |x: i32, y: i32| -> i32 {
        let mut min = std::i32::MAX;
        let mut index = -1;

        for (i, p) in input.iter().enumerate() {
            let distance = (p[0] - x).abs() + (p[1] - y).abs();

            match distance.cmp(&min) {
                Ordering::Less => {
                    min = distance;
                    index = i as i32;
                }
                Ordering::Equal => index = -1,
                _ => {}
            }
        }

        index
    };

    let mut points: HashMap<i32, HashSet<(i32, i32)>> = HashMap::new();
    let mut region: HashSet<(i32, i32)> = HashSet::new();

    for x in input.iter().map(|e| e[0]).min().unwrap()..=input.iter().map(|e| e[0]).max().unwrap() {
        for y in input.iter().map(|e| e[1]).min().unwrap()..=input.iter().map(|e| e[1]).max().unwrap() {
            let closest = get_closest_index_to(x, y);

            if closest >= 0 {
                points.entry(closest).or_default();
                points.get_mut(&closest).unwrap().insert((x, y));
            }

            if get_total_distance_to(x, y) < 10000 {
                region.insert((x, y));
            }
        }
    }

    let result1 = points.iter().map(|e| e.1.len()).max().unwrap();
    let result2 = region.len();

    println!("6\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

#[test]
fn test()
{
    assert_eq!(solve(), (4233, 45290));
}
