use evalexpr::*;

pub fn solve() -> (i64, i64)
{
    let mut monkeys = include_str!("input.txt")
        .split("\n\n")
        .map(|monkey| {
            monkey
                .split("\n")
                .map(|l| {
                    if l.contains("Starting") {
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
                        "0".to_string()
                    }
                })
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

    let lcm = monkeys
        .iter()
        .map(|monkey| monkey[3].parse::<i64>().unwrap())
        .product::<i64>();

    for _ in 1..=10_000 {
        for monkey in &mut monkeys {
            for item in monkey[1].split(",").filter(|x| x != &"") {
                let worry_level = eval(&monkey[2].replace("old", item)).unwrap().as_int().unwrap();

                let next = (worry_level % monkey[3].parse::<i64>().unwrap()).min(1) + 4;

                monkeys[monkey[next as usize].parse::<usize>().unwrap()][1] += &format!(",{}", worry_level % lcm);

                monkey[0] = (monkey[0].parse::<i64>().unwrap() + 1).to_string();
            }

            monkey[1] = "".to_string();
        }
    }

    let mut result = monkeys
        .iter()
        .map(|monkey| monkey[0].parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    result.sort();
    result.reverse();

    let result1 = result.iter().take(2).product::<i64>();

    (result1, 0)
}

#[cfg(test)]
mod tests
{
    #[test]
    fn solve()
    {
        assert_eq!(super::solve(), (2713310158, 28537348205));
    }
}
