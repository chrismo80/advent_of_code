pub fn solve() -> (usize, usize)
{
    let input: Vec<i32> = include_str!("input.txt").lines().map(|x| x.parse().unwrap()).collect();

    let action = |items: &Vec<i32>| items.as_slice().windows(2).map(|w| (w[1] > w[0]) as usize).sum::<usize>();

    let windows: Vec<i32> = input.as_slice().windows(3).map(|w| w.iter().sum::<i32>()).collect();

    let result1 = action(&input);
    let result2 = action(&windows);

    println!("1\t{:<20}\t{:<20}", result1, result2);

    (result1, result2)
}

#[test]
fn test()
{
    assert_eq!(solve(), (1696, 1737));
}
