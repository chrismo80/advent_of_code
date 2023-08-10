use evalexpr::*;
use std::collections::*;

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

    let mut monkeys = std::collections::HashMap::<i32, Monkey>::new();

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

    let lcm = monkeys.values().map(|monkey| monkey.test).product::<i64>();

    for _ in 1..=10_000 {
        for i in 0..monkeys.len() {
            let mut throw_items = HashMap::<i32, Vec<i64>>::new();

            {
                let monkey = &mut monkeys.get_mut(&(i as i32)).unwrap();

                for item in monkey.items.clone() {
                    let worry_level = eval(&monkey.operation.replace("old", &item.to_string()))
                        .unwrap()
                        .as_int()
                        .unwrap();

                    monkey.inspections += 1;

                    if worry_level % monkey.test == 0 {
                        throw_items
                            .entry(monkey.throw_true)
                            .and_modify(|l| l.push(worry_level % lcm))
                            .or_insert(vec![worry_level % lcm]);
                    }
                    else {
                        throw_items
                            .entry(monkey.throw_false)
                            .and_modify(|l| l.push(worry_level % lcm))
                            .or_insert(vec![worry_level % lcm]);
                    }
                }

                monkey.items.clear();
            }

            for receiver in throw_items {
                monkeys
                    .get_mut(&receiver.0)
                    .unwrap()
                    .items
                    .append(&mut receiver.1.clone());
            }
        }
    }

    let mut result = monkeys.values().map(|monkey| monkey.inspections).collect::<Vec<i64>>();

    result.sort();
    result.reverse();

    let result1 = result.iter().take(2).product::<i64>();
    let result2 = 0;

    println!("11\t{result1}\t{result2}");

    (result1, 0)
}

#[cfg(test)]
mod tests
{
    #[test]
    fn solve()
    {
        assert_eq!(super::solve().0, 2713310158);
    }
}
