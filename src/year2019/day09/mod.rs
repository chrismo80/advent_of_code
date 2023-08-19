use super::intcode_computer::*;
use std::collections::*;

pub fn solve() -> (i64, i64)
{
    let input = include_str!("input.txt").split(',');

    let memory: HashMap<i64, i64> = input.enumerate().map(|(i, x)| (i as i64, x.parse().unwrap())).collect();

    let run = |input: i64| {
        let mut icc = IntCodeComputer::new(memory.clone());
        icc.add_input(input);
        icc.run();
        icc.get_output().unwrap()
    };

    let result1 = run(1);
    let result2 = run(2);

    println!("9\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

#[cfg(test)]
mod tests
{
    #[test]
    fn solve()
    {
        assert_eq!(super::solve(), (3454977209, 50120));
    }
}
