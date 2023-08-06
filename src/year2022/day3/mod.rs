use std::collections::HashSet;

pub fn solve()
{
    let input: Vec<&str> = include_str!("input.txt").lines().collect();

    let result1 = part_1(&input);
    let result2 = part_2(&input);

    println!("3\t{result1}\t{result2}");
}

fn part_1(input: &Vec<&str>) -> i32
{
    let mut result1 = 0;

    for line in input {
        let packs = line.split_at(line.len() / 2);

        let left: HashSet<char> = packs.0.chars().collect();
        let right: HashSet<char> = packs.1.chars().collect();

        let overlap = left.intersection(&right).into_iter().next().unwrap().clone();

        result1 += get_priority(overlap);
    }

    result1
}

fn part_2(input: &Vec<&str>) -> i32
{
    let mut result2 = 0;

    for chunks in input.chunks(3).into_iter() {
        let c1: HashSet<char> = chunks[0].chars().collect();
        let c2: HashSet<char> = chunks[1].chars().collect();
        let c3: HashSet<char> = chunks[2].chars().collect();

        let overlap = c1
            .intersection(&c2)
            .into_iter()
            .copied()
            .collect::<HashSet<char>>()
            .intersection(&c3)
            .into_iter()
            .next()
            .unwrap()
            .clone();

        result2 += get_priority(overlap);
    }

    result2
}

fn get_priority(c: char) -> i32
{
    match c.is_uppercase() {
        true => c as i32 - 'A' as i32 + 27,
        false => c as i32 - 'a' as i32 + 1,
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn solve_input()
    {
        let input: Vec<&str> = include_str!("input.txt").lines().collect();

        assert_eq!(part_1(&input), 7674);
        assert_eq!(part_2(&input), 2805);
    }

    #[test]
    fn solve_test()
    {
        let input: Vec<&str> = include_str!("test.txt").lines().collect();

        assert_eq!(part_1(&input), 157);
        assert_eq!(part_2(&input), 70);
    }
}
