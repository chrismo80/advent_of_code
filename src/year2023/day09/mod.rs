use crate::extensions::converter::Parser;

pub fn solve() -> (i64, i64)
{
    let input = include_str!("input.txt").to_vec_of_vec::<i64>("\n", " ");

    let result1 = input.iter().map(|v| predict(v.to_vec())).sum();
    let result2 = input.iter().map(|v| predict(v.iter().rev().copied().collect())).sum();

    println!("9\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

fn predict(mut diffs: Vec<i64>) -> i64
{
    let mut prediction = 0;

    while diffs.iter().any(|&d| d != 0) {
        prediction += update_diffs(&mut diffs);
    }

    prediction
}

fn update_diffs(values: &mut Vec<i64>) -> i64
{
    for i in 0..values.len() - 1 {
        values[i] = values[i + 1] - values[i];
    }

    values.pop().unwrap()
}

#[test]
fn test()
{
    assert_eq!(solve(), (1884768153, 1031));
}
