use crate::extensions::converter::Parser;
use iter_tools::Itertools;

struct Monkey
{
    items: Vec<usize>,
    operation: Box<dyn Fn(usize) -> usize>,
    test: usize,
    throw_true: usize,
    throw_false: usize,
    inspections: usize,
}

impl Monkey
{
    fn throw(&mut self, divide: bool) -> Option<(usize, usize)>
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

    fn catch(&mut self, item: usize)
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
        let item_list = parts[1].split_once(':').unwrap().1.trim().split(", ");

        Ok(Monkey {
            items: item_list.map(|i| i.parse::<usize>().unwrap()).collect::<Vec<usize>>(),
            operation: get_operation(parts[2].split_once('=').unwrap().1.trim().to_string()),
            test: parts[3].split_once('y').unwrap().1.trim().parse::<usize>().unwrap(),
            throw_true: parts[4].split_once('y').unwrap().1.trim().parse::<usize>().unwrap(),
            throw_false: parts[5].split_once('y').unwrap().1.trim().parse::<usize>().unwrap(),
            inspections: 0,
        })
    }
}

pub fn solve() -> (usize, usize)
{
    let result1 = play(include_str!("input.txt").to_vec::<Monkey>("\n\n"), 20);
    let result2 = play(include_str!("input.txt").to_vec::<Monkey>("\n\n"), 10_000);

    println!("11\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

fn play(mut monkeys: Vec<Monkey>, rounds: i32) -> usize
{
    let lcm: usize = monkeys.iter().map(|monkey| monkey.test).product();

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].throw(rounds < 100) {
                monkeys[item.1].catch(item.0 % lcm);
            }
        }
    }

    monkeys.iter().map(|m| m.inspections).sorted().rev().take(2).product()
}

fn get_operation(expression: String) -> Box<dyn Fn(usize) -> usize>
{
    let parts = expression.split_whitespace().collect::<Vec<&str>>();
    let operand = parts[2].to_string();

    match (operand.as_str(), parts[1]) {
        ("old", "+") => Box::new(move |old| old + old),
        ("old", "*") => Box::new(move |old| old * old),
        (_, "+") => Box::new(move |old| old + operand.parse::<usize>().unwrap()),
        (_, "*") => Box::new(move |old| old * operand.parse::<usize>().unwrap()),
        _ => panic!("Unknown operation"),
    }
}

#[test]
fn test()
{
    assert_eq!(solve(), (100345, 28537348205));
}
