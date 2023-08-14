pub fn solve() -> (i32, i32)
{
    let input = include_str!("input.txt").lines().collect::<Vec<&str>>();

    let mut result1 = 0;
    let mut result2 = 0;

    for line in &input {
        let split = line.split(',').collect::<Vec<&str>>();

        let left = split[0].split('-').collect::<Vec<&str>>();
        let right = split[1].split('-').collect::<Vec<&str>>();

        let left_min = left[0].parse::<i32>().unwrap();
        let left_max = left[1].parse::<i32>().unwrap();

        let right_min = right[0].parse::<i32>().unwrap();
        let right_max = right[1].parse::<i32>().unwrap();

        result1 += overlap_fully(left_min, left_max, right_min, right_max) as i32;
        result2 += overlap_at_all(left_min, left_max, right_min, right_max) as i32;
    }

    println!("4\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

fn overlap_at_all(min1: i32, max1: i32, min2: i32, max2: i32) -> bool
{
    min1 <= max2 && max1 >= min2
}

fn overlap_fully(min1: i32, max1: i32, min2: i32, max2: i32) -> bool
{
    min1 <= min2 && max1 >= max2 || min2 <= min1 && max2 >= max1
}

#[cfg(test)]
mod tests
{
    #[test]
    fn solve()
    {
        assert_eq!(super::solve(), (556, 876));
    }
}
