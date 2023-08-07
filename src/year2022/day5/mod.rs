pub fn solve() -> (String, String)
{
    let input = include_str!("test.txt").lines().collect::<Vec<&str>>();

    let mut result1 = "".to_string();
    let mut result2 = "".to_string();

    for line in &input {}

    println!("5\t{result1}\t{result2}");

    (result1, result2)
}

#[cfg(test)]
mod tests
{
    #[test]
    fn verify()
    {
        // MCD
        assert_eq!(super::solve(), ("CMZ".to_string(), "".to_string()));

        // SHMSDGZVC VRZGHDFBQ
    }
}
