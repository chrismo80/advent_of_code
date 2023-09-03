pub trait SimpleParser
{
    fn to_char_grid(&self) -> Vec<Vec<char>>;
}

pub trait GenericParser
{
    fn to_vec_multiline<T: std::str::FromStr>(&self) -> Vec<T>
    where
        <T as std::str::FromStr>::Err: std::fmt::Debug;

    fn to_vec<T: std::str::FromStr>(&self) -> Vec<T>
    where
        <T as std::str::FromStr>::Err: std::fmt::Debug;
}

impl SimpleParser for &str
{
    fn to_char_grid(&self) -> Vec<Vec<char>>
    {
        self.lines().map(|line| line.chars().collect()).collect()
    }
}

impl GenericParser for &str
{
    fn to_vec<T: std::str::FromStr>(&self) -> Vec<T>
    where
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.lines().map(|line| line.parse::<T>().unwrap()).collect()
    }

    fn to_vec_multiline<T: std::str::FromStr>(&self) -> Vec<T>
    where
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.split("\n\n").map(|line| line.parse::<T>().unwrap()).collect()
    }
}
