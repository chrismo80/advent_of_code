use fasteval::*;
use std::collections::*;

#[derive(Clone, Debug)]
struct Monkey
{
    items: Vec<i64>,
    operation: String,
    test: i64,
    throw_true: i32,
    throw_false: i32,
    inspections: i64,
}

pub fn solve() -> (i64, i64)
{
    let input = include_str!("test.txt")
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

    let mut monkeys = HashMap::<i32, Monkey>::new();

    for chunk in input {
        monkeys.insert(
            chunk[0].parse::<i32>().unwrap(),
            Monkey {
                items: chunk[1]
                    .split(", ")
                    .map(|i| i.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>(),
                operation: chunk[2].clone(),
                test: chunk[3].parse::<i64>().unwrap(),
                throw_true: chunk[4].parse::<i32>().unwrap(),
                throw_false: chunk[5].parse::<i32>().unwrap(),
                inspections: 0,
            },
        );
    }

    let lcm: i64 = monkeys.values().map(|monkey| monkey.test).product();

    let result1 = play(monkeys.clone(), lcm, 20);
    let result2 = play(monkeys.clone(), lcm, 10_000);

    println!("11\t{result1}\t{result2}");

    (result1, result2)
}

fn play(mut monkeys: HashMap<i32, Monkey>, lcm: i64, rounds: i32) -> i64
{
    let mut ns = fasteval::EmptyNamespace;

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let mut items_true: (i32, Vec<i64>) = (-1, Vec::new());
            let mut items_false: (i32, Vec<i64>) = (-1, Vec::new());

            {
                let monkey = monkeys.get_mut(&(i as i32)).unwrap();

                monkey.inspections += monkey.items.len() as i64;

                items_true.0 = monkey.throw_true;
                items_false.0 = monkey.throw_false;

                while let Some(item) = monkey.items.pop() {
                    let mut worry_level =
                        fasteval::ez_eval(&monkey.operation.replace("old", format!("{}", item).as_str()), &mut ns)
                            .unwrap() as i64;

                    if rounds < 1000 {
                        worry_level /= 3;
                    }

                    if worry_level % monkey.test == 0 {
                        items_true.1.push(worry_level % lcm);
                    }
                    else {
                        items_false.1.push(worry_level % lcm);
                    }
                }
            }

            for item in items_true.1 {
                monkeys.get_mut(&items_true.0).unwrap().items.push(item);
            }

            for item in items_false.1 {
                monkeys.get_mut(&items_false.0).unwrap().items.push(item);
            }
        }
    }

    let mut result: Vec<i64> = monkeys.values().map(|monkey| monkey.inspections).collect();

    result.sort();

    result.iter().rev().take(2).product::<i64>()
}

#[cfg(test)]
mod tests
{
    #[test]
    fn solve()
    {
        //assert_eq!(super::solve(), (10605, 2713310158)); // 10197

        assert_eq!(super::solve(), (100345, 28537348205)); // 105210
    }
}
