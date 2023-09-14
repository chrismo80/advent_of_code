pub fn solve() -> (i64, i64)
{
    let input: Vec<(&str, i64)> = include_str!("input.txt")
        .lines()
        .map(|l| l.split_whitespace())
        .map(|mut l| (l.next().unwrap(), l.next().unwrap().parse::<i64>().unwrap()))
        .collect();

    let result1 = run_program(&input).0;

    let mut i = 0;

    let result2 = loop {
        let mut program = input.clone();
        i += 1;

        match program[i] {
            ("jmp", value) => program[i] = ("nop", value),
            ("nop", value) => program[i] = ("jmp", value),
            _ => continue,
        }

        let (acc, terminated) = run_program(&program);

        if terminated {
            break acc;
        }
    };

    println!("8\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

fn run_program(program: &Vec<(&str, i64)>) -> (i64, bool)
{
    let mut acc = 0;
    let mut pointer = 0;
    let mut executed = vec![false; program.len()];

    while pointer < program.len() && !executed[pointer] {
        executed[pointer] = true;

        match program[pointer] {
            ("acc", value) => acc += value,
            ("jmp", value) => pointer = (pointer as i64 + value - 1) as usize,
            _ => (),
        }
        pointer += 1;
    }

    (acc, pointer >= program.len())
}

#[test]
fn test()
{
    assert_eq!(solve(), (1709, 1976));
}
