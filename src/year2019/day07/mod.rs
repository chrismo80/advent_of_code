use super::intcode_computer::*;
use crate::extensions::{converter::Converter, permutations::*};

pub fn solve() -> (i64, i64)
{
    let memory = include_str!("input.txt").to_vec::<i64>(",");

    let result = |phase_setting: &str| {
        phase_setting
            .chars()
            .collect::<Vec<char>>()
            .permutations()
            .iter()
            .map(|setting| amplifier_chain(memory.clone(), setting.clone()))
            .max()
    };

    let result1 = result("01234").unwrap();
    let result2 = result("56789").unwrap();

    println!("7\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

fn amplifier_chain(memory: Vec<i64>, phases: Vec<char>) -> i64
{
    let mut amps = Vec::<IntCodeComputer>::new();

    for i in 0..phases.len() {
        amps.push(IntCodeComputer::new(&memory));
        amps[i].add_input(phases[i].to_digit(10).unwrap() as i64);
    }
    amps[0].add_input(0);

    let count = phases.len();

    let mut states: Vec<State> = (0..count).map(|i| amps[i].run()).collect();

    while states.iter().any(|state| *state == State::Waiting) {
        for i in 0..count {
            if states[i] == State::Waiting {
                if let Some(input) = amps[(count + i - 1) % count].get_output() {
                    amps[i].add_input(input);
                    states[i] = amps[i].run();
                }
            }
        }
    }

    amps.last_mut().unwrap().get_output().unwrap()
}

#[test]
fn test()
{
    assert_eq!(solve(), (17440, 27561242));
}
