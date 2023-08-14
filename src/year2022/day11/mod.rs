struct Monkey
{
    items: Vec<i64>,
    operation: Box<dyn Fn(i64) -> i64>,
    test: i64,
    throw_true: usize,
    throw_false: usize,
    inspections: usize,
}

impl Monkey
{
    fn throw(&mut self, divide: bool) -> Option<(i64, usize)>
    {
        let item = self.items.pop()?;

        self.inspections += 1;

        let mut worry_level = (self.operation)(item);

        if divide {
            worry_level /= 3;
        }

        match worry_level % self.test == 0 {
            true => Some((worry_level, self.throw_true)),
            false => Some((worry_level, self.throw_false)),
        }
    }

    fn catch(&mut self, item: i64)
    {
        self.items.push(item);
    }
}

impl std::str::FromStr for Monkey
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        let parts = s.lines().collect::<Vec<&str>>();

        Ok(Monkey {
            items: parts[1]
                .split_once(':')
                .unwrap()
                .1
                .trim()
                .split(", ")
                .map(|i| i.parse::<i64>().unwrap())
                .collect::<Vec<i64>>(),
            operation: get_operation(parts[2].split_once('=').unwrap().1.trim().to_string()),
            test: parts[3].split_once('y').unwrap().1.trim().parse::<i64>().unwrap(),
            throw_true: parts[4].split_once('y').unwrap().1.trim().parse::<usize>().unwrap(),
            throw_false: parts[5].split_once('y').unwrap().1.trim().parse::<usize>().unwrap(),
            inspections: 0,
        })
    }
}

pub fn solve() -> (usize, usize)
{
    let input: Vec<&str> = include_str!("input.txt").split("\n\n").collect();

    let result1 = play(input.iter().map(|m| m.parse::<Monkey>().unwrap()).collect(), 20);
    let result2 = play(input.iter().map(|m| m.parse::<Monkey>().unwrap()).collect(), 10_000);

    println!("11\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

fn play(mut monkeys: Vec<Monkey>, rounds: i32) -> usize
{
    let lcm: i64 = monkeys.iter().map(|monkey| monkey.test).product();

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].throw(rounds < 1000) {
                monkeys[item.1].catch(item.0 % lcm);
            }
        }
    }

    let mut result: Vec<usize> = monkeys.iter().map(|monkey| monkey.inspections).collect();

    result.sort();

    result.iter().rev().take(2).product::<usize>()
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
