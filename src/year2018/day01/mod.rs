use std::collections::HashSet;

pub fn solve() -> (i32, i32)
{
    let input: Vec<i32> = include_str!("input.txt").lines().map(|x| x.parse().unwrap()).collect();

    let result1 = input.iter().sum();

    let mut frequency = 0;
    let mut history: HashSet<i32> = HashSet::new();
    let mut i = 0;

    while history.insert(frequency) {
        frequency += input[i];

        i += 1;
        i %= input.len();
    }

    let result2 = frequency;

    println!("1\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

#[test]
fn test()
{
    assert_eq!(solve(), (590, 83445));
}
