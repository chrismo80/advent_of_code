use crate::extensions::converter::Parser;

pub fn solve() -> (usize, usize)
{
    let input = include_str!("input.txt").to_vec::<usize>(",");

    let result1 = population_after(&input, 80);
    let result2 = population_after(&input, 256);

    println!("6\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

fn population_after(parents: &[usize], days: usize) -> usize
{
    let mut children = vec![0; days];

    for &age in parents {
        for birthday in birthdays(age, days) {
            children[birthday] += 1;
        }
    }

    for day in 0..days {
        for birthday in birthdays(8, days - day - 1) {
            children[day + 1 + birthday] += children[day];
        }
    }

    children.iter().sum::<usize>() + parents.len()
}

fn birthdays(age: usize, days: usize) -> Vec<usize>
{
    (0..days / 6).map(|x| x * 7 + age).filter(|&x| x < days).collect()
}

#[test]
fn test()
{
    assert_eq!(solve(), (360268, 1632146183902));
}
