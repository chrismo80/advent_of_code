pub fn solve() -> (String, String)
{
    let input: Vec<&str> = include_str!("input.txt").lines().collect();

    let mut result1 = String::from("");
    let mut result2 = String::from("");

    let mut key1 = 5;
    let mut key2 = 5;

    for line in &input {
        for c in line.chars() {
            key1 = match c {
                'D' if key1 < 7 => key1 + 3,
                'L' if (key1 - 1) % 3 != 0 => key1 - 1,
                'U' if key1 > 3 => key1 - 3,
                'R' if key1 % 3 != 0 => key1 + 1,
                _ => key1,
            };

            key2 = match c {
                'U' if ![5, 2, 1, 4, 9].contains(&key2) => key2 - if [3, 13].contains(&key2) { 2 } else { 4 },
                'D' if ![5, 10, 13, 12, 9].contains(&key2) => key2 + if [1, 11].contains(&key2) { 2 } else { 4 },
                'L' if ![1, 2, 5, 10, 13].contains(&key2) => key2 - 1,
                'R' if ![1, 4, 9, 12, 13].contains(&key2) => key2 + 1,
                _ => key2,
            };
        }

        result1 += key1.to_string().as_str();
        result2 += format!("{:X}", key2).as_str();
    }

    println!("2\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

#[cfg(test)]
mod tests
{
    #[test]
    fn solve()
    {
        assert_eq!(super::solve(), ("33444".to_string(), "446A6".to_string()));
    }
}
