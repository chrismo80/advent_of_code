use crate::extensions::converter::Converter;

pub fn solve() -> (i64, i64)
{
    let mut ranges = include_str!("input.txt").to_vec_of_vec::<i64>("\n", "-");

    ranges.sort();

    let mut result1 = 0;
    let mut result2 = 0;
    let mut last_blocked = 0;

    for range in ranges {
        if range[0] - last_blocked > 1 {
            result2 += range[0] - last_blocked - 1;
        }

        if result1 == 0 && result2 > 0 {
            result1 = last_blocked + 1
        };

        last_blocked = last_blocked.max(range[1]);
    }

    result2 += 2_i64.pow(32) - 1 - last_blocked; // 4294967295 - lastBlocked

    println!("20\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

#[test]
fn test()
{
    assert_eq!(solve(), (32259706, 113));
}
