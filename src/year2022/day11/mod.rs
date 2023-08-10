pub fn solve() -> (usize, usize)
{
    let mut monkeys = include_str!("input.txt")
        .split("\n\n")
        .map(|monkey| {
            monkey
                .split("\n")
                .map(|l| {
                    if l.contains("Starting") {
                        l.split("items: ").into_iter().last().unwrap().to_string()
                    }
                    else if l.contains("Operation:") {
                        l.split("new = ").into_iter().last().unwrap().to_string()
                    }
                    else if l.contains("Test:") {
                        l.split("divisible by ").into_iter().last().unwrap().to_string()
                    }
                    else if l.contains("If") {
                        l.split("throw to monkey ").into_iter().last().unwrap().to_string()
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
        .map(|monkey| monkey[3].parse::<usize>().unwrap())
        .product::<usize>();

    for _ in 1..=10_000 {
        for mut monkey in monkeys {
            for item in monkey[1].split(",").filter(|x| x != &"") {
                let worry_level = monkey[2]
                    .replace("old", &format!("{}.0", item))
                    .parse::<usize>()
                    .unwrap();

                let next = (worry_level % monkey[3].parse::<usize>().unwrap()).min(1) + 4;

                monkeys[monkey[next].parse::<usize>().unwrap()][1] += &format!(",{}", worry_level % lcm);

                monkey[0] = (monkey[0].parse::<usize>().unwrap() + 1).to_string();
            }

            monkey[1] = "".to_string();
        }
    }

    let mut result = monkeys
        .iter()
        .map(|monkey| monkey[0].parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    result.sort();
    result.reverse();

    let result1 = result.iter().take(2).product::<usize>();

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
