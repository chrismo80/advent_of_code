use super::intcode_computer::*;

pub fn solve() -> (i64, i64)
{
    let input = include_str!("input.txt").split(',');

    let memory: Vec<i64> = input.map(|x| x.parse().unwrap()).collect();

    let run = |input: i64| {
        let mut icc = IntCodeComputer::new(&memory);
        icc.add_input(input);
        icc.run();
        *icc.outputs.iter().last().unwrap()
    };

    let result1 = run(1);
    let result2 = run(5);

    println!("5\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

#[test]
fn test()
{
    assert_eq!(solve(), (7265618, 7731427));
}
