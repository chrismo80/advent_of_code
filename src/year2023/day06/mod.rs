use crate::extensions::converter::Parser;

pub fn solve() -> (usize, usize)
{
    let mut input = include_str!("input.txt").lines();

    let times = input.next().unwrap().to_vec::<usize>(" ");
    let distances = input.next().unwrap().to_vec::<usize>(" ");

    let result1 = (0..times.len()).map(|i| count(times[i], distances[i])).product();
    let result2 = count(merge(times), merge(distances));

    println!("6\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

fn merge(v: Vec<usize>) -> usize
{
    v.iter().map(|x| x.to_string()).collect::<String>().parse::<usize>().unwrap()
}

fn count(time: usize, record: usize) -> usize
{
    let last = (time / 2..time).find(|speed| speed * (time - speed) < record).unwrap();

    2 * last - time - 1
}

#[test]
fn test()
{
    assert_eq!(solve(), (219849, 29432455));
}
