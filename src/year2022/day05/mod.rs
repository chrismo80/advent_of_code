use std::collections::HashMap;

pub fn solve() -> (String, String)
{
    let input = include_str!("input.txt").lines().collect::<Vec<&str>>();

    let mut stacks = HashMap::<usize, Vec<char>>::new();
    let mut stacks_9000 = HashMap::<usize, Vec<char>>::new();
    let mut stacks_9001 = HashMap::<usize, Vec<char>>::new();

    for line in &input {
        if line.starts_with("move") {
            let split = line.split_whitespace().collect::<Vec<&str>>();

            let count = split[1].parse::<usize>().unwrap();
            let from = split[3].parse::<usize>().unwrap();
            let to = split[5].parse::<usize>().unwrap();

            mover_9000(&mut stacks_9000, count, from, to);
            mover_9001(&mut stacks_9001, count, from, to);
        }
        else if line.is_empty() {
            stacks_9000 = stacks.clone();
            stacks_9001 = stacks.clone();
        }
        else {
            for (i, chunk) in line.chars().collect::<Vec<char>>().chunks(4).enumerate() {
                let stack = stacks.entry(i + 1).or_default();
                let item = chunk[1];

                if item.is_numeric() {
                    stack.reverse();
                    continue;
                }

                if item != ' ' {
                    stack.push(item);
                }
            }
        }
    }

    let mut res1: Vec<char> = Vec::new();
    let mut res2: Vec<char> = Vec::new();

    for i in 1..=stacks.len() {
        res1.push(stacks_9000.entry(i).or_default().pop().unwrap());
        res2.push(stacks_9001.entry(i).or_default().pop().unwrap());
    }

    let result1 = res1.iter().collect::<String>();
    let result2 = res2.iter().collect::<String>();

    println!("5\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

fn mover_9000(stacks: &mut HashMap<usize, Vec<char>>, count: usize, from: usize, to: usize)
{
    for _ in 0..count {
        let stack = stacks.entry(from).or_default();
        let item = stack.pop().unwrap();
        stacks.entry(to).or_default().push(item);
    }
}

fn mover_9001(stacks: &mut HashMap<usize, Vec<char>>, count: usize, from: usize, to: usize)
{
    let stack = stacks.entry(from).or_default();
    let mut bulk: Vec<char> = Vec::new();

    (0..count).for_each(|_| bulk.push(stack.pop().unwrap()));

    bulk.reverse();
    bulk.iter().for_each(|item| stacks.entry(to).or_default().push(*item));
}

#[test]
fn test()
{
    assert_eq!(solve(), ("SHMSDGZVC".to_string(), "VRZGHDFBQ".to_string()));
}
