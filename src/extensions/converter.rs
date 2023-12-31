use std::str::FromStr;

pub trait Matrix
{
    type Item;

    fn transpose(&self) -> Vec<Vec<Self::Item>>;
    fn rotate(&self) -> Vec<Vec<Self::Item>>;
    fn flip(&self) -> Vec<Vec<Self::Item>>;
    fn print(&self);
}

impl<T: Copy + std::fmt::Display> Matrix for Vec<Vec<T>>
{
    type Item = T;

    fn transpose(&self) -> Vec<Vec<T>>
    {
        (0..self[0].len()).map(|i| self.iter().map(|col| col[i]).collect()).collect()
    }

    fn rotate(&self) -> Vec<Vec<T>>
    {
        self.transpose().iter().map(|r| r.iter().rev().copied().collect()).collect()
    }

    fn flip(&self) -> Vec<Vec<T>>
    {
        self.iter().map(|r| r.iter().rev().copied().collect()).collect()
    }

    fn print(&self)
    {
        println!();
        for row in self {
            for item in row {
                print!("{}", item);
            }
            println!();
        }
        println!();
    }
}
pub trait Parser
{
    fn to_char_grid(&self) -> Vec<Vec<char>>;
    fn to_int_grid(&self) -> Vec<Vec<u32>>;
    fn to_matrix(&self, delim: &str) -> Vec<Vec<u32>>;
    fn to_vec<T: FromStr>(&self, delim: &str) -> Vec<T>;
    fn to_vec_of_vec<T: FromStr>(&self, delim1: &str, delim2: &str) -> Vec<Vec<T>>;
    fn to_vec_of_vec_of_vec<T: FromStr>(&self, delim1: &str, delim2: &str, delim3: &str) -> Vec<Vec<Vec<T>>>;
    fn to_vec_from_regex(&self, pattern: &str) -> Vec<Vec<&str>>;
}

impl Parser for &str
{
    fn to_char_grid(&self) -> Vec<Vec<char>>
    {
        self.lines().map(|l| l.chars().collect()).collect()
    }

    fn to_int_grid(&self) -> Vec<Vec<u32>>
    {
        self.lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect()
    }

    fn to_matrix(&self, delim: &str) -> Vec<Vec<u32>>
    {
        self.lines()
            .map(|l| l.split(delim).filter_map(|e| e.parse().ok()).collect())
            .collect()
    }

    fn to_vec<T: FromStr>(&self, delim: &str) -> Vec<T>
    {
        self.split(delim).filter_map(|e| e.parse().ok()).collect()
    }

    fn to_vec_of_vec<T: FromStr>(&self, delim1: &str, delim2: &str) -> Vec<Vec<T>>
    {
        self.split(delim1)
            .map(|e| e.to_vec(delim2))
            .filter(|e| !e.is_empty())
            .collect()
    }

    fn to_vec_of_vec_of_vec<T: FromStr>(&self, delim1: &str, delim2: &str, delim3: &str) -> Vec<Vec<Vec<T>>>
    {
        self.split(delim1)
            .map(|e| e.to_vec_of_vec(delim2, delim3))
            .filter(|e| !e.is_empty())
            .collect()
    }

    fn to_vec_from_regex(&self, pattern: &str) -> Vec<Vec<&str>>
    {
        let re = regex::Regex::new(pattern).unwrap();

        let mut list = vec![];

        for line in self.lines() {
            if let Some(groups) = re.captures(line) {
                list.push((1..groups.len()).map(|i| groups.get(i).unwrap().as_str()).collect());
            }
        }

        list
    }
}
