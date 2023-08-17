pub fn solve() -> (i32, i32)
{
    let input: Vec<i32> = include_str!("input.txt")
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let result1 = run(&mut input.clone(), 1);
    let result2 = run(&mut input.clone(), 5);

    println!("5\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

fn run(program: &mut [i32], input: i32) -> i32
{
    let mut i = 0;
    let mut output = 0;

    while program[i] != 99 {
        let instruction = program[i].to_string();

        if instruction.ends_with('1') {
            let mode1 = format!("{:0>5}", instruction).chars().rev().nth(2).unwrap();
            let mode2 = format!("{:0>5}", instruction).chars().rev().nth(3).unwrap();

            let param1 = if mode1 == '0' {
                program[program[i + 1] as usize]
            }
            else {
                program[i + 1]
            };
            let param2 = if mode2 == '0' {
                program[program[i + 2] as usize]
            }
            else {
                program[i + 2]
            };

            program[program[i + 3] as usize] = param1 + param2;
            i += 4;
        }
        else if instruction.ends_with('2') {
            let mode1 = format!("{:0>5}", instruction).chars().rev().nth(2).unwrap();
            let mode2 = format!("{:0>5}", instruction).chars().rev().nth(3).unwrap();

            let param1 = if mode1 == '0' {
                program[program[i + 1] as usize]
            }
            else {
                program[i + 1]
            };
            let param2 = if mode2 == '0' {
                program[program[i + 2] as usize]
            }
            else {
                program[i + 2]
            };

            program[program[i + 3] as usize] = param1 * param2;
            i += 4;
        }
        else if instruction == "3" {
            program[program[i + 1] as usize] = input;
            i += 2;
        }
        else if instruction.ends_with('4') {
            let mode1 = format!("{:0>5}", instruction).chars().rev().nth(2).unwrap();
            output = if mode1 == '0' {
                program[program[i + 1] as usize]
            }
            else {
                program[i + 1]
            };
            i += 2;
        }
        else if input == 1 {
            continue;
        }
        else if instruction.ends_with('5') {
            let mode1 = format!("{:0>5}", instruction).chars().rev().nth(2).unwrap();
            let mode2 = format!("{:0>5}", instruction).chars().rev().nth(3).unwrap();

            let param1 = if mode1 == '0' {
                program[program[i + 1] as usize]
            }
            else {
                program[i + 1]
            };
            let param2 = if mode2 == '0' {
                program[program[i + 2] as usize]
            }
            else {
                program[i + 2]
            };

            i = if param1 != 0 { param2 as usize } else { i + 3 };
        }
        else if instruction.ends_with('6') {
            let mode1 = format!("{:0>5}", instruction).chars().rev().nth(2).unwrap();
            let mode2 = format!("{:0>5}", instruction).chars().rev().nth(3).unwrap();

            let param1 = if mode1 == '0' {
                program[program[i + 1] as usize]
            }
            else {
                program[i + 1]
            };
            let param2 = if mode2 == '0' {
                program[program[i + 2] as usize]
            }
            else {
                program[i + 2]
            };

            i = if param1 == 0 { param2 as usize } else { i + 3 };
        }
        else if instruction.ends_with('7') {
            let mode1 = format!("{:0>5}", instruction).chars().rev().nth(2).unwrap();
            let mode2 = format!("{:0>5}", instruction).chars().rev().nth(3).unwrap();

            let param1 = if mode1 == '0' {
                program[program[i + 1] as usize]
            }
            else {
                program[i + 1]
            };
            let param2 = if mode2 == '0' {
                program[program[i + 2] as usize]
            }
            else {
                program[i + 2]
            };

            program[program[i + 3] as usize] = if param1 < param2 { 1 } else { 0 };
            i += 4;
        }
        else if instruction.ends_with('8') {
            let mode1 = format!("{:0>5}", instruction).chars().rev().nth(2).unwrap();
            let mode2 = format!("{:0>5}", instruction).chars().rev().nth(3).unwrap();

            let param1 = if mode1 == '0' {
                program[program[i + 1] as usize]
            }
            else {
                program[i + 1]
            };
            let param2 = if mode2 == '0' {
                program[program[i + 2] as usize]
            }
            else {
                program[i + 2]
            };

            program[program[i + 3] as usize] = if param1 == param2 { 1 } else { 0 };
            i += 4;
        }
    }

    output
}

#[cfg(test)]
mod tests
{
    #[test]
    fn solve()
    {
        assert_eq!(super::solve(), (7265618, 7731427));
    }
}
