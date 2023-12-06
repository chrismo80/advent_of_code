use crate::extensions::converter::Parser;

pub fn solve() -> (usize, usize)
{
    let mut input = include_str!("input.txt").lines();

    let times = input.next().unwrap().to_vec::<usize>(" ");
    let distances = input.next().unwrap().to_vec::<usize>(" ");

    let time = times.iter().map(|t| t.to_string()).collect::<String>();
    let distance = distances.iter().map(|t| t.to_string()).collect::<String>();

    let result1 = (0..times.len()).map(|i| count(times[i], distances[i])).product();
    let result2 = count(time.parse().unwrap(), distance.parse().unwrap());

    println!("6\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

fn count(time: usize, record: usize) -> usize
{
    let start = (1..time).find(|speed| speed * (time - speed) > record).unwrap();
    let end = (start..time).find(|speed| speed * (time - speed) < record).unwrap();

    end - start
}

#[test]
fn test()
{
    assert_eq!(solve(), (219849, 29432455));
}
