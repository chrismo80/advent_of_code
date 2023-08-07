use std::collections::HashSet;

pub fn solve() -> (i32, i32)
{
    let input: Vec<&str> = include_str!("input.txt").lines().collect();

    let mut result1 = 0;

    for line in &input {
        let packs = line.split_at(line.len() / 2);

        let left: HashSet<char> = packs.0.chars().collect();
        let right: HashSet<char> = packs.1.chars().collect();

        let overlap = left.intersection(&right).next().unwrap().clone();

        result1 += get_priority(overlap);
    }

    let mut result2 = 0;

    for chunks in input.chunks(3).into_iter() {
        let c1: HashSet<char> = chunks[0].chars().collect();
        let c2: HashSet<char> = chunks[1].chars().collect();
        let c3: HashSet<char> = chunks[2].chars().collect();

        let overlap = c1
            .intersection(&c2)
            .copied()
            .collect::<HashSet<char>>()
            .intersection(&c3)
            .next()
            .unwrap()
            .clone();

        result2 += get_priority(overlap);
    }

    println!("3\t{result1}\t{result2}");

    (result1, result2)
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
    #[test]
    fn verify()
    {
        assert_eq!(super::solve(), (7674, 2805));
    }
}
