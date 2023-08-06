use std::collections::*;
use to_vec::*;

pub fn solve()
{
    let input = include_str!("test.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    for line in input.iter() {
        let mut chunks = line.chunks(line.len() / 2);

        let left = chunks.next().iter().to_set();
        let right = chunks.next().iter().to_set();

        let u = left.intersection(&right).to_set();

        println!("{:?}", left);
        println!("{:?}", right);

        println!("{:?}", u);
        println!("");
    }

    let result1 = 0;
    let result2 = 0;

    println!("3\t{result1}\t{result2}");
}
