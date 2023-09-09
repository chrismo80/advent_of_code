use super::intcode_computer::*;
use crate::extensions::converter::Converter;

pub fn solve() -> (i64, i64)
{
    let memory = include_str!("input.txt").to_vec::<i64>(",");

    let run = |input: i64| {
        let mut icc = IntCodeComputer::new(&memory);
        icc.add_input(input);
        icc.run();
        icc.get_output().unwrap()
    };

    let result1 = run(1);
    let result2 = run(2);

    println!("9\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

#[test]
fn test()
{
    assert_eq!(solve(), (3454977209, 50120));
}
