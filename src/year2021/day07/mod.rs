use crate::extensions::converter::Converter;

pub fn solve() -> (i64, i64)
{
    let mut input = include_str!("input.txt").to_vec::<i64>(",");

    input.sort();

    let closure1 = |t: i64, x: i64| (t - x).abs();
    let closure2 = |t: i64, x: i64| {
        let end = (t - x).abs();
        (end * (end + 1)) / 2 // 1+2+3+4+5+...+n = (n*(n+1))/2
    };

    let result1 = calc(&input, closure1);
    let result2 = calc(&input, closure2);

    println!("7\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

fn calc<C>(input: &[i64], closure: C) -> i64
where
    C: Fn(i64, i64) -> i64,
{
    (input[0]..*input.last().unwrap() + 1)
        .map(|t| input.iter().map(|&x| closure(t, x)).sum())
        .min()
        .unwrap()
}

#[test]
fn test()
{
    assert_eq!(solve(), (328262, 90040997));
}
