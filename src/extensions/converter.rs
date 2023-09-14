use std::str::FromStr;

pub trait Converter
{
    fn to_char_grid(&self) -> Vec<Vec<char>>;
    fn to_int_grid(&self) -> Vec<Vec<u32>>;
    fn to_vec<T: FromStr>(&self, delim: &str) -> Vec<T>;
    fn to_vec_of_vec<T: FromStr>(&self, delim1: &str, delim2: &str) -> Vec<Vec<T>>;
    fn to_vec_of_vec_of_vec<T: FromStr>(&self, delim1: &str, delim2: &str, delim3: &str) -> Vec<Vec<Vec<T>>>;
}

impl Converter for &str
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

    fn to_vec<T: FromStr>(&self, delim: &str) -> Vec<T>
    {
        self.split(delim).filter_map(|e| e.parse::<T>().ok()).collect()
    }

    fn to_vec_of_vec<T: FromStr>(&self, delim1: &str, delim2: &str) -> Vec<Vec<T>>
    {
        self.split(delim1).map(|e| e.to_vec::<T>(delim2)).collect()
    }

    fn to_vec_of_vec_of_vec<T: FromStr>(&self, delim1: &str, delim2: &str, delim3: &str) -> Vec<Vec<Vec<T>>>
    {
        self.split(delim1).map(|e| e.to_vec_of_vec::<T>(delim2, delim3)).collect()
    }
}
