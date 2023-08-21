use crate::extensions::permutations::*;

use super::intcode_computer::*;
use std::collections::*;

pub fn solve() -> (i64, i64)
{
    let input = include_str!("input.txt").split(',');

    let memory: HashMap<i64, i64> = input.enumerate().map(|(i, x)| (i as i64, x.parse().unwrap())).collect();

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

fn amplifier_chain(memory: HashMap<i64, i64>, phases: Vec<char>) -> i64
{
    let mut amps: Vec<IntCodeComputer> = phases.iter().map(|phase| IntCodeComputer::new(memory.clone())).collect();

    for i in 0..phases.len() {
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

#[cfg(test)]
mod tests
{
    #[test]
    fn solve()
    {
        assert_eq!(super::solve(), (17440, 27561242));
    }
}
