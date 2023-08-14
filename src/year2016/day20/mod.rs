use iter_tools::Itertools;

pub fn solve() -> (i64, i64)
{
    let ranges: Vec<(i64, i64)> = include_str!("input.txt")
        .lines()
        .map(|row| row.split('-').map(|n| n.parse::<i64>().unwrap()).collect::<Vec<i64>>())
        .map(|range| (range[0], range[1]))
        .sorted()
        .collect();

    let mut result1 = 0;
    let mut result2 = 0;
    let mut last_blocked = 0;

    for (min, max) in ranges {
        if min - last_blocked > 1 {
            result2 += min - last_blocked - 1;
        }

        if result1 == 0 && result2 > 0 {
            result1 = last_blocked + 1
        };

        last_blocked = last_blocked.max(max);
    }

    result2 += 2_i64.pow(32) - 1 - last_blocked; // 4294967295 - lastBlocked

    println!("20\t{result1:<15}\t{result2:<15}");

    (result1, result2)
}

#[cfg(test)]
mod tests
{
    #[test]
    fn solve()
    {
        assert_eq!(super::solve(), (32259706, 113));
    }
}
