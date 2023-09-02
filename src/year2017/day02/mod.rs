pub fn solve() -> (i32, i32)
{
    let input: Vec<Vec<i32>> = include_str!("input.txt")
        .lines()
        .map(|line| line.split('\t').map(|item| item.parse().unwrap()).collect())
        .collect();

    let result1 = input.iter().map(|l| l.iter().max().unwrap() - l.iter().min().unwrap()).sum();
    let result2 = input.iter().map(|l| find_numers(l)).sum();

    println!("2\t{:<20}\t{:<20}", result1, result2);

    (result1, result2)
}

fn find_numers(input: &[i32]) -> i32
{
    for &x in input.iter() {
        for &y in input.iter() {
            if x != y && x % y == 0 {
                return x / y;
            }
        }
    }
    unreachable!()
}

#[test]
fn test()
{
    assert_eq!(solve(), (41919, 303));
}
