use crate::extensions::count_items::CountItems;
use iter_tools::Itertools;

pub fn solve() -> (i32, String)
{
    let input: Vec<&str> = include_str!("input.txt").lines().collect();

    let result1 = count(&input, 2) * count(&input, 3);
    let result2 = find_similar(&input);

    println!("2\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

fn count(input: &[&str], appearences: i32) -> i32
{
    let mut count = 0;

    for box_id in input {
        let chars: Vec<char> = box_id.chars().collect();

        if chars.count_items().values().contains(&appearences) {
            count += 1;
        }
    }

    count
}

fn find_similar(input: &[&str]) -> String
{
    let mut result = "".to_owned();

    for (a, b) in input.iter().tuple_combinations() {
        let mut diff = 0;
        let mut index = 0;

        for (i, (a, b)) in a.chars().zip(b.chars()).enumerate() {
            if a != b {
                diff += 1;
                index = i;
            }
        }

        if diff == 1 {
            result = a.chars().take(index).chain(a.chars().skip(index + 1)).collect();
            break;
        }
    }

    result
}

#[test]
fn test()
{
    assert_eq!(solve(), (8715, "fvstwblgqkhpuixdrnevmaycd".to_owned()));
}
