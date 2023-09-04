use crate::extensions::input_parser::*;

pub fn solve() -> (usize, usize)
{
    let input = include_str!("input.txt").to_vec::<i32>("\n");

    let mut program = input.clone();
    let mut pointer = 0;
    let mut steps = 0;
    let change = 1;

    while pointer < program.len() as i32 {
        program[pointer as usize] += change;
        pointer += program[pointer as usize] - change;
        steps += 1;
    }

    let result1 = steps;

    program = input;
    pointer = 0;
    steps = 0;

    while pointer < program.len() as i32 {
        let current = &mut program[pointer as usize];
        pointer += *current;
        *current += if *current >= 3 { -1 } else { 1 };
        steps += 1;
    }

    let result2 = steps;

    println!("5\t{:<20}\t{:<20}", result1, result2);

    (result1, result2)
}

#[test]
fn test()
{
    assert_eq!(solve(), (358131, 25558839));
}
