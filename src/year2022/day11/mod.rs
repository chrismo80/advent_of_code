struct Monkey
{
    items: Vec<i64>,
    operation: Box<dyn Fn(i64) -> i64>,
    test: i64,
    throw_true: usize,
    throw_false: usize,
    inspections: usize,
}

pub fn solve() -> (usize, usize)
{
    let input = include_str!("input.txt")
        .split("\n\n")
        .map(|monkey| {
            monkey
                .split('\n')
                .map(|l| {
                    if l.starts_with("Monkey") {
                        l[7..8].to_string()
                    }
                    else if l.contains("Starting") {
                        l.split("items: ").last().unwrap().to_string()
                    }
                    else if l.contains("Operation:") {
                        l.split("new = ").last().unwrap().to_string()
                    }
                    else if l.contains("Test:") {
                        l.split("divisible by ").last().unwrap().to_string()
                    }
                    else if l.contains("If") {
                        l.split("throw to monkey ").last().unwrap().to_string()
                    }
                    else {
                        "-".to_string()
                    }
                })
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

    let result1 = play(create_monkeys(&input), 20);
    let result2 = play(create_monkeys(&input), 10_000);

    println!("11\t{result1}\t{result2}");

    (result1, result2)
}

fn play(mut monkeys: Vec<Monkey>, rounds: i32) -> usize
{
    let lcm: i64 = monkeys.iter().map(|monkey| monkey.test).product();

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];

            monkey.inspections += monkey.items.len();

            let mut items_true = vec![];
            let mut items_false = vec![];

            while let Some(item) = monkey.items.pop() {
                let mut worry_level = (monkey.operation)(item);

                if rounds < 1000 {
                    worry_level /= 3;
                }

                if worry_level % monkey.test == 0 {
                    items_true.push(worry_level % lcm);
                }
                else {
                    items_false.push(worry_level % lcm);
                }
            }

            let monkey_true = monkey.throw_true;
            let monkey_false = monkey.throw_false;

            monkeys[monkey_true].items.extend(items_true);
            monkeys[monkey_false].items.extend(items_false);
        }
    }

    let mut result: Vec<usize> = monkeys.iter().map(|monkey| monkey.inspections).collect();

    result.sort();

    result.iter().rev().take(2).product::<usize>()
}

fn create_monkeys(input: &Vec<Vec<String>>) -> Vec<Monkey>
{
    let mut monkeys: Vec<Monkey> = Vec::new();

    for chunk in input {
        monkeys.push(Monkey {
            items: chunk[1]
                .split(", ")
                .map(|i| i.parse::<i64>().unwrap())
                .collect::<Vec<i64>>(),
            operation: get_operation(chunk[2].to_string()),
            test: chunk[3].parse::<i64>().unwrap(),
            throw_true: chunk[4].parse::<usize>().unwrap(),
            throw_false: chunk[5].parse::<usize>().unwrap(),
            inspections: 0,
        });
    }

    monkeys
}

fn get_operation(expression: String) -> Box<dyn Fn(i64) -> i64>
{
    let parts = expression.split_whitespace().collect::<Vec<&str>>();
    let operand = parts[2].to_string();

    if operand == "old" {
        match parts[1] {
            "+" => Box::new(move |old| old + old),
            "*" => Box::new(move |old| old * old),
            _ => panic!("Unknown operation"),
        }
    }
    else {
        match parts[1] {
            "+" => Box::new(move |old| old + operand.parse::<i64>().unwrap()),
            "*" => Box::new(move |old| old * operand.parse::<i64>().unwrap()),
            _ => panic!("Unknown operation"),
        }
    }
}

#[cfg(test)]
mod tests
{
    #[test]
    fn solve()
    {
        assert_eq!(super::solve(), (100345, 28537348205));
    }
}
