use crate::extensions::count_items::CountItems;

pub fn solve() -> (usize, usize)
{
    let mut input = include_str!("input.txt").lines();

    let arrival = input.next().unwrap().parse::<usize>().unwrap();

    let buses: Vec<(usize, usize)> = input
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|(_, id)| id != &"x")
        .map(|(lane, id)| (lane, id.parse::<usize>().unwrap()))
        .collect();

    let (id, time) = buses
        .iter()
        .map(|(_, id)| (id, (arrival / id * id) + id))
        .min_by_key(|(_, time)| *time)
        .unwrap();

    let mut timestamp = 0;

    let mut matches: Vec<&(usize, usize)> = buses.iter().filter(|(lane, id)| (timestamp + lane) % id == 0).collect();

    while matches.len() < buses.len() {
        timestamp += matches.iter().map(|m| m.1).product::<usize>();
        matches = buses.iter().filter(|(lane, id)| (timestamp + lane) % id == 0).collect();
    }

    let result1 = (time - arrival) * id;
    let result2 = timestamp;

    println!("13\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

#[cfg(test)]
mod tests
{
    #[test]
    fn solve()
    {
        assert_eq!(super::solve(), (207, 530015546283687));
    }
}
