struct ListEntry
{
    left: usize,
    right: usize,
    char: char,
    password: String,
}

impl std::str::FromStr for ListEntry
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        let parts = s.split_once(": ").unwrap();
        let specs = parts.0.split_once(' ').unwrap();
        let range = specs.0.split_once('-').unwrap();

        Ok(ListEntry {
            left: range.0.parse().unwrap(),
            right: range.1.parse::<usize>().unwrap(),
            char: specs.1.chars().next().unwrap(),
            password: parts.1.to_string(),
        })
    }
}

impl ListEntry
{
    fn is_valid_old_policy(&self) -> bool
    {
        let count = self.password.chars().filter(|c| *c == self.char).count();
        count >= self.left && count <= self.right
    }

    fn is_valid_new_policy(&self) -> bool
    {
        (self.password.chars().nth(self.left - 1).unwrap() == self.char)
            ^ (self.password.chars().nth(self.right - 1).unwrap() == self.char)
    }
}

pub fn solve() -> (usize, usize)
{
    let input: Vec<&str> = include_str!("input.txt").lines().collect();

    let list: Vec<ListEntry> = input.iter().map(|line| line.parse().unwrap()).collect();

    let result1 = list.iter().filter(|e| e.is_valid_old_policy()).count();
    let result2 = list.iter().filter(|e| e.is_valid_new_policy()).count();

    println!("2\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

#[cfg(test)]
mod tests
{
    #[test]
    fn solve()
    {
        assert_eq!(super::solve(), (418, 616));
    }
}
