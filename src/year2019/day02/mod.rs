use crate::extensions::converter::Parser;

pub fn solve() -> (i32, i32)
{
    let input = include_str!("input.txt").to_vec::<i32>(",");

    let result1 = run_program(&mut input.clone(), 12, 2);
    let mut result2 = 0;

    for noun in 0..100 {
        for verb in 0..100 {
            if run_program(&mut input.clone(), noun, verb) == 19690720 {
                result2 = noun * 100 + verb;
                break;
            }
        }
    }

    println!("2\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

fn run_program(program: &mut [i32], noun: i32, verb: i32) -> i32
{
    program[1] = noun;
    program[2] = verb;

    for i in (0..program.len()).step_by(4) {
        if program[i] == 99 {
            return program[0];
        }

        if program[i] == 1 {
            program[program[i + 3] as usize] = program[program[i + 1] as usize] + program[program[i + 2] as usize];
        }

        if program[i] == 2 {
            program[program[i + 3] as usize] = program[program[i + 1] as usize] * program[program[i + 2] as usize];
        }
    }

    -1
}

#[test]
fn test()
{
    assert_eq!(solve(), (5534943, 7603));
}
