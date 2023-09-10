use crate::extensions::converter::Converter;
use std::collections::*;

pub fn solve() -> (usize, usize)
{
    let mut memory = include_str!("input.txt").to_vec::<u32>("\t");

    let size = memory.len();
    let mut history = HashMap::new();
    let mut counter = 0;

    loop {
        if let Some(position) = history.insert(memory.clone(), counter) {
            counter = position;
            break;
        }

        let mut value = *memory.iter().max().unwrap();
        let mut pos = memory.iter().position(|&x| x == value).unwrap();

        counter += 1;
        memory[pos] = 0;

        while value > 0 {
            value -= 1;
            pos += 1;
            memory[pos % size] += 1;
        }
    }

    let result1 = history.len();
    let result2 = history.len() - counter;

    println!("6\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

#[test]
fn test()
{
    assert_eq!(solve(), (12841, 8038));
}
